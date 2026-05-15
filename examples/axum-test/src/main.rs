use std::{
    env,
    process::{Child, Command, Stdio},
    sync::Arc,
};

use axum::{
    extract::Query,
    extract::State,
    http::{Method, StatusCode},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use solari_core::modules::vipps::{models::VippsConfig, provider::VippsProvider};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

const WORKSPACE_ENV_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.env");
const EXPO_APP_RETURN_URL_PREFIX: &str = "solari-expo-test://";

#[derive(Clone)]
struct AppState {
    vipps_provider: Arc<VippsProvider>,
    payment_state: Arc<RwLock<PaymentSnapshot>>,
}

struct NgrokProcessGuard {
    child: Child,
}

impl Drop for NgrokProcessGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

const VIPPS_PAY_AMOUNT_NOK: u32 = 67;

#[derive(Debug, Clone, Serialize)]
struct PaymentSnapshot {
    provider: String,
    status: String,
    requested_amount: u32,
    paid_amount: u32,
    reference: Option<String>,
    redirect_url: Option<String>,
    return_url: Option<String>,
    attempts: u64,
    updated_from: String,
    last_error: Option<String>,
    last_webhook_payload: Option<Value>,
}

impl Default for PaymentSnapshot {
    fn default() -> Self {
        Self {
            provider: "vipps".to_string(),
            status: "idle".to_string(),
            requested_amount: 0,
            paid_amount: 0,
            reference: None,
            redirect_url: None,
            return_url: None,
            attempts: 0,
            updated_from: "startup".to_string(),
            last_error: None,
            last_webhook_payload: None,
        }
    }
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    ok: bool,
    payment: PaymentSnapshot,
}

#[derive(Debug, Deserialize)]
struct PayRequestBody {
    return_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VippsReturnQuery {
    app_return_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path(WORKSPACE_ENV_PATH).ok();

    let listen_port = read_env_u16("AXUM_PORT", 3001)?;
    let ngrok_domain = read_env_string("NGROK_DOMAIN");
    let _ngrok_guard = start_ngrok_tunnel(listen_port)?;

    let vipps_provider = VippsProvider::new(reqwest::Client::new(), vipps_config_from_env()?);

    let state = AppState {
        vipps_provider: Arc::new(vipps_provider),
        payment_state: Arc::new(RwLock::new(PaymentSnapshot::default())),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(home_screen))
        .route("/health", get(health))
        .route("/vipps-return", get(vipps_return))
        .route("/pay", post(pay))
        .route("/status", get(status))
        .route("/wipe", post(wipe))
        .route("/webhook/vipps", post(vipps_webhook))
        .route("/wehbook/vipps", post(vipps_webhook))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", listen_port))
        .await
        .unwrap_or_else(|_| panic!("failed to bind 0.0.0.0:{listen_port}"));

    println!("http://127.0.0.1:{listen_port}");

    if let Some(domain) = ngrok_domain {
        println!("{domain}");
    }

    axum::serve(listener, app).await?;

    Ok(())
}

fn start_ngrok_tunnel(port: u16) -> Result<Option<NgrokProcessGuard>, Box<dyn std::error::Error>> {
    if !read_env_bool("NGROK_ENABLED", false) {
        return Ok(None);
    }

    let mut command = Command::new("ngrok");
    command
        .arg("http")
        .arg(port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    let ngrok_domain = env::var("NGROK_DOMAIN")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    if let Some(domain) = &ngrok_domain {
        let ngrok_domain_flag = domain
            .strip_prefix("https://")
            .or_else(|| domain.strip_prefix("http://"))
            .unwrap_or(domain)
            .trim_end_matches('/')
            .to_string();

        command.arg(format!("--domain={ngrok_domain_flag}"));
        println!("ngrok tunnel domain configured: {domain}");
    } else {
        println!(
            "ngrok enabled without NGROK_DOMAIN (ephemeral URL). Visit ngrok dashboard/terminal output to copy the tunnel URL."
        );
    }

    if let Ok(token) = env::var("NGROK_AUTHTOKEN") {
        if !token.trim().is_empty() {
            command.arg(format!("--authtoken={token}"));
        }
    }

    let child = command.spawn().map_err(|error| {
        format!(
            "failed to start ngrok. Ensure ngrok is installed and in PATH. underlying error: {error}"
        )
    })?;

    println!("ngrok process started");
    Ok(Some(NgrokProcessGuard { child }))
}

fn read_env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => default,
    }
}

fn read_env_string(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn read_env_u16(key: &str, default: u16) -> Result<u16, Box<dyn std::error::Error>> {
    match env::var(key) {
        Ok(value) => {
            let parsed = value
                .trim()
                .parse::<u16>()
                .map_err(|e| format!("invalid {key}: {e}"))?;
            Ok(parsed)
        }
        Err(_) => Ok(default),
    }
}

fn required_env(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var(key).map_err(|_| format!("missing required env var: {key}").into())
}

fn vipps_config_from_env() -> Result<VippsConfig, Box<dyn std::error::Error>> {
    Ok(VippsConfig::new(
        required_env("VIPPS_BASE_URL")?,
        required_env("VIPPS_CLIENT_ID")?,
        required_env("VIPPS_CLIENT_SECRET")?,
        required_env("VIPPS_SUBSCRIPTION_KEY")?,
        required_env("VIPPS_MSN")?,
    ))
}

async fn home_screen() -> Html<String> {
    Html(
        r#"
    <h1><b>Solari</b> test api<h1>
    "#
        .to_string(),
    )
}

async fn health(State(_state): State<AppState>) -> &'static str {
    "healthy"
}

fn sanitize_app_return_url(candidate: Option<String>) -> Option<String> {
    let candidate = candidate
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())?;

    if candidate.starts_with(EXPO_APP_RETURN_URL_PREFIX)
        || candidate.starts_with("https://")
        || candidate.starts_with("http://localhost")
        || candidate.starts_with("http://127.0.0.1")
        || candidate.starts_with("http://[::1]")
    {
        return Some(candidate);
    }

    None
}

async fn vipps_return(Query(query): Query<VippsReturnQuery>) -> Html<String> {
    if let Some(app_return_url) = sanitize_app_return_url(query.app_return_url) {
        return Html(format!(
            r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>Returning to app</title>
                    <meta http-equiv="refresh" content="0;url={app_return_url}" />
                </head>
                <body>
                    <p>Returning to app...</p>
                    <script>
                        window.location.replace({app_return_url:?});
                    </script>
                </body>
            </html>
            "#
        ));
    }

    Html(
        r#"
        <!doctype html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>Payment complete</title>
                <style>
                    body {
                        margin: 0;
                        min-height: 100vh;
                        display: grid;
                        place-items: center;
                        font-family: system-ui, sans-serif;
                        background: #fff8ef;
                        color: #23160d;
                    }

                    .card {
                        width: min(92vw, 420px);
                        border-radius: 14px;
                        background: white;
                        box-shadow: 0 12px 28px rgba(35, 22, 13, 0.12);
                        padding: 24px;
                        text-align: center;
                    }

                    h1 {
                        margin: 0 0 10px;
                        color: #ff5b24;
                        font-size: 24px;
                    }

                    p {
                        margin: 0;
                        line-height: 1.5;
                    }
                </style>
            </head>
            <body>
                <section class="card">
                    <h1>Payment complete</h1>
                    <p>You can return to the original app or browser tab.</p>
                </section>
                <script>
                    const openedAsPopup = !!window.opener && !window.opener.closed;

                    if (openedAsPopup) {
                        try {
                            window.opener.postMessage({ type: "solari-vipps-return", ok: true }, "*");
                        } catch (_) {
                            // Ignore cross-window messaging issues.
                        }

                        window.close();
                    }
                </script>
            </body>
        </html>
        "#
        .to_string(),
    )
}

async fn pay(
    State(state): State<AppState>,
    payload: Option<Json<PayRequestBody>>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    let return_url = payload.and_then(|body| body.return_url.clone());

    {
        let mut snapshot = state.payment_state.write().await;
        snapshot.attempts += 1;
        snapshot.provider = "vipps".to_string();
        snapshot.status = "creating".to_string();
        snapshot.requested_amount = VIPPS_PAY_AMOUNT_NOK;
        snapshot.paid_amount = 0;
        snapshot.reference = None;
        snapshot.redirect_url = None;
        snapshot.return_url = return_url.clone();
        snapshot.updated_from = "/pay".to_string();
        snapshot.last_error = None;
    }

    let response = state
        .vipps_provider
        .create_payment(VIPPS_PAY_AMOUNT_NOK, return_url.as_deref())
        .await
        .map_err(|err| {
            let message = err.to_string();
            (StatusCode::BAD_GATEWAY, message)
        });

    let mut snapshot = state.payment_state.write().await;

    match response {
        Ok(response) => {
            snapshot.status = "pending".to_string();
            snapshot.requested_amount = VIPPS_PAY_AMOUNT_NOK;
            snapshot.paid_amount = 0;
            snapshot.reference = response.reference;
            snapshot.redirect_url = response.redirect_url;
            snapshot.return_url = return_url;
            snapshot.updated_from = "/pay".to_string();
            snapshot.last_error = None;

            Ok(Json(ApiResponse {
                ok: true,
                payment: snapshot.clone(),
            }))
        }
        Err((status, message)) => {
            eprintln!("/pay failed: {status} {message}");
            snapshot.status = "failed".to_string();
            snapshot.requested_amount = VIPPS_PAY_AMOUNT_NOK;
            snapshot.paid_amount = 0;
            snapshot.reference = None;
            snapshot.redirect_url = None;
            snapshot.return_url = return_url;
            snapshot.updated_from = "/pay".to_string();
            snapshot.last_error = Some(message.clone());
            Err((status, message))
        }
    }
}

async fn status(State(state): State<AppState>) -> Json<ApiResponse> {
    let snapshot = state.payment_state.read().await.clone();

    Json(ApiResponse {
        ok: true,
        payment: snapshot,
    })
}

async fn wipe(State(state): State<AppState>) -> Json<ApiResponse> {
    let mut snapshot = state.payment_state.write().await;
    *snapshot = PaymentSnapshot {
        updated_from: "/wipe".to_string(),
        ..PaymentSnapshot::default()
    };

    Json(ApiResponse {
        ok: true,
        payment: snapshot.clone(),
    })
}

async fn vipps_webhook(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<ApiResponse> {
    let mut snapshot = state.payment_state.write().await;
    snapshot.provider = "vipps".to_string();
    snapshot.status = infer_status_from_webhook(&payload).to_string();
    snapshot.updated_from = "/webhook/vipps".to_string();
    snapshot.last_webhook_payload = Some(payload);
    snapshot.last_error = None;

    Json(ApiResponse {
        ok: true,
        payment: snapshot.clone(),
    })
}

fn infer_status_from_webhook(payload: &Value) -> &'static str {
    let candidate = payload
        .pointer("/status")
        .and_then(Value::as_str)
        .or_else(|| payload.pointer("/state").and_then(Value::as_str))
        .or_else(|| payload.pointer("/eventType").and_then(Value::as_str))
        .or_else(|| payload.pointer("/name").and_then(Value::as_str))
        .or_else(|| payload.pointer("/eventName").and_then(Value::as_str))
        .unwrap_or("pending")
        .to_ascii_lowercase();

    if candidate.contains("cancel") {
        "cancelled"
    } else if candidate.contains("fail") || candidate.contains("deny") {
        "failed"
    } else if candidate.contains("capture")
        || candidate.contains("complete")
        || candidate.contains("authorize")
        || candidate.contains("paid")
    {
        "completed"
    } else {
        "pending"
    }
}
