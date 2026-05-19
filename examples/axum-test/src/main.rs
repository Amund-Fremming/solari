use std::{env, sync::Arc};

use solari::{app_router_with_vipps, app_router_with_vipps_and_stripe};
use solari::{SolariPaymentService, StripeConfig, VippsConfig};
use tokio::sync::RwLock;
use tracing_subscriber::{fmt, EnvFilter};

mod handlers;
mod models;
mod ngrok;

use handlers::create_router;
use models::{AppState, PaymentSnapshot, PaymentState};
use ngrok::start_ngrok_tunnel;

const WORKSPACE_ENV_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.env");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .compact()
        .init();

    dotenvy::from_path(WORKSPACE_ENV_PATH).ok();

    let listen_port = read_env_u16("AXUM_PORT", 3001)?;
    let ngrok_domain = read_env_string("NGROK_DOMAIN");
    let _ngrok_guard = start_ngrok_tunnel(listen_port)?;

    let mut payment_module = SolariPaymentService::new()?;
    payment_module.vipps(vipps_config_from_env()?);

    let state = AppState {
        payment_module: Arc::new(payment_module),
        payment_state: Arc::new(RwLock::new(PaymentState {
            snapshot: PaymentSnapshot::default(),
        })),
    };

    let vipps_config = vipps_config_from_env()?;
    let stripe_config = stripe_config_from_env_if_present()?;

    let solari_api_router = if let Some(stripe_config) = stripe_config {
        println!("solari routes enabled: /solari/pay, /solari/status, /solari/webhooks/vipps");
        app_router_with_vipps_and_stripe(vipps_config, stripe_config)
            .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?
    } else {
        println!("stripe disabled: /solari/pay still works for vipps; set STRIPE_SECRET_KEY to enable stripe/apple_pay provider in /solari/pay");
        app_router_with_vipps(vipps_config)
            .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?
    };

    let app = create_router(state).merge(solari_api_router);

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

fn stripe_config_from_env_if_present() -> Result<Option<StripeConfig>, Box<dyn std::error::Error>> {
    let maybe_secret = read_env_string("STRIPE_SECRET_KEY");
    if maybe_secret.is_none() {
        return Ok(None);
    }

    let api_base_url = read_env_string("STRIPE_API_BASE_URL")
        .unwrap_or_else(|| "https://api.stripe.com".to_string());
    let publishable_key = required_env("STRIPE_PUBLISHABLE_KEY")?;
    let webhook_secret = required_env("STRIPE_WEBHOOK_SECRET")?;
    let account_id = read_env_string("STRIPE_ACCOUNT_ID");

    Ok(Some(StripeConfig::new(
        api_base_url,
        maybe_secret.expect("checked is_some"),
        publishable_key,
        webhook_secret,
        account_id,
    )))
}
