use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use solari_core::{payment_module::VippsPayRequest, PayRequest, PaymentStatus};

use crate::{
    error::{ApiResult, SolariApiError},
    models::{
        AppState, VippsCreatePaymentResponseBody, VippsPayBody, VippsStatusQuery,
        VippsStatusResponseBody,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pay", post(pay))
        .route("/status", get(status))
}

async fn pay(
    State(state): State<AppState>,
    Json(body): Json<VippsPayBody>,
) -> ApiResult<Json<VippsCreatePaymentResponseBody>> {
    let payment = state
        .payment_service
        .pay(PayRequest::Vipps(VippsPayRequest {
            amount: body.amount,
            return_url: body.return_url,
        }))
        .await?;

    Ok(Json(VippsCreatePaymentResponseBody {
        reference: payment.reference,
        redirect_url: payment.redirect_url,
    }))
}

async fn status(
    State(state): State<AppState>,
    Query(query): Query<VippsStatusQuery>,
) -> ApiResult<Json<VippsStatusResponseBody>> {
    let reference = query.reference.trim();
    if reference.is_empty() {
        return Err(SolariApiError::BadRequest(
            "missing or empty 'reference' query parameter".to_string(),
        ));
    }

    let result = state.payment_service.vipps_get_payment_status(reference).await?;

    Ok(Json(VippsStatusResponseBody {
        reference: result.reference,
        raw_status: result.raw_status,
        status: match result.status {
            PaymentStatus::Pending => "pending",
            PaymentStatus::Completed => "completed",
            PaymentStatus::Failed => "failed",
            PaymentStatus::Cancelled => "cancelled",
        }
        .to_string(),
    }))
}
