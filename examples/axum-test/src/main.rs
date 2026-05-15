use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use solari_core::{
    core::PaymentType,
    modules::{
        apple_pay::models::ApplePayConfig, stripe::models::StripeConfig, vipps::models::VippsConfig,
    },
    payment_module::PaymentModule,
};

#[derive(Clone)]
struct AppState {
    payment_module: Arc<PaymentModule>,
}

#[derive(Debug, Deserialize)]
struct PayRequest {
    amount: u32,
}

#[derive(Debug, Serialize)]
struct PayResponse {
    provider: String,
    status: String,
    paid: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut payment_module = PaymentModule::new()?;

    payment_module
        .vipps(VippsConfig::new(
            "https://api.vipps.no".to_string(),
            "demo-client-id".to_string(),
            "demo-client-secret".to_string(),
            "demo-subscription-key".to_string(),
            "demo-merchant-serial-number".to_string(),
        ))
        .apple_pay(ApplePayConfig::new(
            "merchant.com.example.solari".to_string(),
            "Solari Demo".to_string(),
            "web".to_string(),
            "localhost".to_string(),
            "https://apple-pay-gateway.apple.com/paymentservices/startSession".to_string(),
            "demo-cert".to_string(),
            "demo-key".to_string(),
        ))
        .stripe(StripeConfig::new(
            "https://api.stripe.com".to_string(),
            "sk_test_demo".to_string(),
            "pk_test_demo".to_string(),
            "whsec_demo".to_string(),
            None,
        ));

    let state = AppState {
        payment_module: Arc::new(payment_module),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/pay/:payment_type", post(pay))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("failed to bind 0.0.0.0:3001");

    println!("axum-test listening on http://0.0.0.0:3001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health(State(_state): State<AppState>) -> &'static str {
    "healthy"
}

async fn pay(
    State(state): State<AppState>,
    Path(payment_type): Path<String>,
    Json(payload): Json<PayRequest>,
) -> Result<Json<PayResponse>, (StatusCode, String)> {
    let provider = parse_payment_type(&payment_type)?;

    let response = state
        .payment_module
        .pay(provider, payload.amount)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

    Ok(Json(PayResponse {
        provider: provider.to_string(),
        status: format!("{:?}", response.status),
        paid: response.paid,
    }))
}

fn parse_payment_type(input: &str) -> Result<PaymentType, (StatusCode, String)> {
    match input.trim().to_ascii_lowercase().as_str() {
        "vipps" => Ok(PaymentType::Vipps),
        "apple_pay" | "applepay" | "apple-pay" => Ok(PaymentType::ApplePay),
        "stripe" => Ok(PaymentType::Stripe),
        _ => Err((
            StatusCode::BAD_REQUEST,
            format!(
                "unsupported payment type '{input}'. expected one of: vipps, apple_pay, stripe"
            ),
        )),
    }
}
