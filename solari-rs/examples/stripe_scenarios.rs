use std::{env, error::Error};

use solari::{PayRequest, SolariPaymentService, StripeConfig, StripePayRequest};

fn required_env(key: &str) -> Result<String, Box<dyn Error>> {
    env::var(key).map_err(|_| format!("missing required env var: {key}").into())
}

fn optional_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn has_required_stripe_env() -> bool {
    [
        "STRIPE_SECRET_KEY",
        "STRIPE_PUBLISHABLE_KEY",
        "STRIPE_WEBHOOK_SECRET",
    ]
    .iter()
    .all(|key| {
        env::var(key)
            .ok()
            .is_some_and(|value| !value.trim().is_empty())
    })
}

fn stripe_config_from_env() -> Result<StripeConfig, Box<dyn Error>> {
    Ok(StripeConfig::new(
        optional_env("STRIPE_API_BASE_URL").unwrap_or_else(|| "https://api.stripe.com".to_string()),
        required_env("STRIPE_SECRET_KEY")?,
        required_env("STRIPE_PUBLISHABLE_KEY")?,
        required_env("STRIPE_WEBHOOK_SECRET")?,
        optional_env("STRIPE_ACCOUNT_ID"),
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    if !has_required_stripe_env() {
        println!(
            "Skipping Stripe scenario: set STRIPE_SECRET_KEY, STRIPE_PUBLISHABLE_KEY, and STRIPE_WEBHOOK_SECRET to run it."
        );
        return Ok(());
    }

    let mut service = SolariPaymentService::new()?;
    service.stripe(stripe_config_from_env()?);

    let response = service
        .pay(PayRequest::Stripe(StripePayRequest {
            amount: 2500,
            currency: Some("nok".to_string()),
            description: Some("Solari Stripe smoke example".to_string()),
        }))
        .await?;

    println!(
        "Stripe pay call completed: provider={}, status={}, reference={:?}",
        response.provider,
        match response.status {
            solari::PaymentStatus::Pending => "pending",
            solari::PaymentStatus::Completed => "completed",
            solari::PaymentStatus::Failed => "failed",
            solari::PaymentStatus::Cancelled => "cancelled",
        },
        response.reference
    );

    Ok(())
}
