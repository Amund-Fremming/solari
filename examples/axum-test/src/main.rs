use std::{env, sync::Arc};

use solari_client::app_router_with_vipps;
use solari_core::{SolariPaymentService, VippsConfig};
use tokio::sync::RwLock;

mod handlers;
mod models;
mod ngrok;

use handlers::create_router;
use models::{AppState, PaymentSnapshot, PaymentState};
use ngrok::start_ngrok_tunnel;

const WORKSPACE_ENV_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.env");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let app = create_router(state).merge(
        app_router_with_vipps(vipps_config_from_env()?)
            .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?,
    );

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
