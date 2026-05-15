use std::{env, error::Error};

use solari_core::modules::vipps::{
    models::{CachedToken, VippsConfig},
    provider::VippsProvider,
};
use tracing::{error, info};

fn required_env(key: &str) -> Result<String, Box<dyn Error>> {
    env::var(key).map_err(|_| format!("missing required env var: {key}").into())
}

async fn get_access_token_scenario() -> Result<CachedToken, Box<dyn Error>> {
    let config = VippsConfig::new(
        required_env("VIPPS_BASE_URL")?,
        required_env("VIPPS_CLIENT_ID")?,
        required_env("VIPPS_CLIENT_SECRET")?,
        required_env("VIPPS_SUBSCRIPTION_KEY")?,
        required_env("VIPPS_MSN")?,
    );

    let client = reqwest::Client::new();
    let provider = VippsProvider::new(client, config);
    let token = provider.fetch_access_token().await?;

    Ok(token)
}

async fn token_cache_scenario() -> Result<(), Box<dyn Error>> {
    let config = VippsConfig::new(
        required_env("VIPPS_BASE_URL")?,
        required_env("VIPPS_CLIENT_ID")?,
        required_env("VIPPS_CLIENT_SECRET")?,
        required_env("VIPPS_SUBSCRIPTION_KEY")?,
        required_env("VIPPS_MSN")?,
    );

    let client = reqwest::Client::new();
    let provider = VippsProvider::new(client, config);

    let first_token = provider.get_valid_token().await?;
    let first_expiry = first_token.expires_at;

    let second_token = provider.get_valid_token().await?;
    let second_expiry = second_token.expires_at;

    // Verify cache is working
    if first_expiry == second_expiry && first_token.token == second_token.token {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "Tokens differ: first_expiry={first_expiry}, second_expiry={second_expiry}, first_token_prefix={}..., second_token_prefix={}...",
            first_token.token.chars().take(8).collect::<String>(),
            second_token.token.chars().take(8).collect::<String>()
        ))
        .into())
    }
}

async fn create_payment_scenario(amount: u32) -> Result<(), Box<dyn Error>> {
    let config = VippsConfig::new(
        required_env("VIPPS_BASE_URL")?,
        required_env("VIPPS_CLIENT_ID")?,
        required_env("VIPPS_CLIENT_SECRET")?,
        required_env("VIPPS_SUBSCRIPTION_KEY")?,
        required_env("VIPPS_MSN")?,
    );

    let client = reqwest::Client::new();
    let provider = VippsProvider::new(client, config);
    let response = provider.create_payment(amount, None).await?;

    info!(
        "Vipps payment scenario response: reference={:?}, redirect_url={:?}",
        response.reference, response.redirect_url
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("🔐 Scenario 1: Get Access Token");
    match get_access_token_scenario().await {
        Ok(cached) => {
            let token_preview: String = cached.token.chars().take(8).collect();
            info!(
                "Vipps access token: {token_preview}... expires at: {}",
                cached.expires_at
            );
        }
        Err(e) => error!("get_access_token_scenario failed: {e}"),
    }

    info!("");
    info!("🔐 Scenario 2: Token Cache");
    match token_cache_scenario().await {
        Ok(_) => {
            info!("Both token cache responses had the same expiry");
        }
        Err(e) => error!("token_cache_scenario failed: {e}"),
    }

    info!("");
    info!("🔐 Scenario 3: Create Payment");
    match create_payment_scenario(100).await {
        Ok(_) => {
            info!("Create payment scenario completed");
        }
        Err(e) => error!("create_payment_scenario failed: {e}"),
    }

    info!("");
    info!("✅ All scenarios finished");
    Ok(())
}
