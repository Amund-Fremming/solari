use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use solari::{PayRequest, PaymentStatus, PaymentType, VippsPayRequest};
use tower_http::cors::{Any, CorsLayer};

use crate::models::{ApiResponse, AppState, PaymentSnapshot, PaymentState, VIPPS_PAY_AMOUNT_NOK};

#[derive(Debug, Deserialize)]
struct PayRequestBody {
    amount: Option<u32>,
    return_url: Option<String>,
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/", get(home_screen))
        .route("/pay", post(pay))
        .route("/status", get(status))
        .route("/wipe", post(wipe))
        .layer(cors)
        .with_state(state)
}

async fn home_screen() -> Html<String> {
    Html(
        r#"
    <h1><b>Solari</b> test api<h1>
    "#
        .to_string(),
    )
}

async fn pay(
    State(state): State<AppState>,
    payload: Option<Json<PayRequestBody>>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    let amount = payload
        .as_ref()
        .and_then(|body| body.amount)
        .unwrap_or(VIPPS_PAY_AMOUNT_NOK);
    let return_url = payload
        .as_ref()
        .and_then(|body| body.return_url.clone())
        .filter(|value| !value.trim().is_empty());

    let current_attempt = {
        let mut payment_state = state.payment_state.write().await;
        payment_state.snapshot.attempts += 1;
        payment_state.snapshot.attempts
    };

    let response = state
        .payment_module
        .pay(PayRequest::Vipps(VippsPayRequest {
            amount,
            return_url: return_url.clone(),
        }))
        .await
        .map_err(|err| {
            let message = err.to_string();
            (StatusCode::BAD_GATEWAY, message)
        });

    let mut payment_state = state.payment_state.write().await;
    let snapshot = &mut payment_state.snapshot;

    match response {
        Ok(response) => {
            snapshot.provider = payment_type_to_str(response.provider).to_string();
            snapshot.status = match response.status {
                PaymentStatus::Pending => "pending",
                PaymentStatus::Completed => "completed",
                PaymentStatus::Failed => "failed",
                PaymentStatus::Cancelled => "cancelled",
            }
            .to_string();
            snapshot.requested_amount = amount;
            snapshot.paid_amount = response.paid;
            snapshot.reference = response.reference;
            snapshot.redirect_url = response.redirect_url;
            snapshot.return_url = response.return_url;
            snapshot.attempts = current_attempt;
            snapshot.updated_from = "/pay".to_string();
            snapshot.last_error = None;
            snapshot.raw_status = None;

            Ok(Json(ApiResponse {
                ok: true,
                payment: snapshot.clone(),
            }))
        }
        Err((status, message)) => {
            eprintln!("/pay failed: {status} {message}");
            snapshot.provider = "vipps".to_string();
            snapshot.status = "failed".to_string();
            snapshot.requested_amount = amount;
            snapshot.paid_amount = 0;
            snapshot.reference = None;
            snapshot.redirect_url = None;
            snapshot.return_url = return_url;
            snapshot.attempts = current_attempt;
            snapshot.updated_from = "/pay".to_string();
            snapshot.last_error = Some(message.clone());
            snapshot.raw_status = None;
            Err((status, message))
        }
    }
}

async fn status(State(state): State<AppState>) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    let mut payment_state = state.payment_state.write().await;
    let reference = payment_state.snapshot.reference.clone();

    if let Some(reference) = reference {
        let status_result = state
            .payment_module
            .vipps_get_payment_status(&reference)
            .await
            .map_err(|err| (StatusCode::BAD_GATEWAY, err.to_string()))?;

        payment_state.snapshot.provider = "vipps".to_string();
        payment_state.snapshot.status = match status_result.status {
            PaymentStatus::Pending => "pending",
            PaymentStatus::Completed => "completed",
            PaymentStatus::Failed => "failed",
            PaymentStatus::Cancelled => "cancelled",
        }
        .to_string();
        payment_state.snapshot.reference = status_result.reference;
        payment_state.snapshot.updated_from = "/status".to_string();
        payment_state.snapshot.last_error = None;
        payment_state.snapshot.raw_status = Some(status_result.raw_status);
    }

    Ok(Json(ApiResponse {
        ok: true,
        payment: payment_state.snapshot.clone(),
    }))
}

async fn wipe(State(state): State<AppState>) -> Json<ApiResponse> {
    let mut payment_state = state.payment_state.write().await;
    *payment_state = PaymentState {
        snapshot: PaymentSnapshot {
            updated_from: "/wipe".to_string(),
            ..PaymentSnapshot::default()
        },
    };

    Json(ApiResponse {
        ok: true,
        payment: payment_state.snapshot.clone(),
    })
}

fn payment_type_to_str(payment_type: PaymentType) -> &'static str {
    match payment_type {
        PaymentType::Vipps => "vipps",
        PaymentType::ApplePay => "apple_pay",
        PaymentType::Stripe => "stripe",
    }
}
