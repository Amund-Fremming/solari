#[cfg(any(feature = "stripe", feature = "vipps"))]
use axum::response::IntoResponse;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::info;

use crate::{
    models::{
        AppState, GenericPayBody, GenericPaymentResponse, GenericStatusQuery, PaymentProvider,
        WebhookEvent,
    },
    PayRequest, SolariError, StripePayRequest, VippsPayRequest,
};

#[cfg(feature = "vipps")]
use crate::adapters::vipps::models::VippsWebhookPayload;

#[cfg(feature = "stripe")]
use crate::adapters::stripe::models::StripeWebhookPayload;

pub fn router() -> Router<AppState> {
    let router = Router::new()
        .route("/pay/{payment_provider}", post(pay))
        .route("/status/{payment_provider}", get(status));

    #[cfg(feature = "vipps")]
    let router = router.route("/webhooks/vipps", post(vipps_webhook));
    #[cfg(feature = "stripe")]
    let router = router.route("/webhooks/stripe", post(stripe_webhook));
    router
}

async fn pay(
    State(state): State<AppState>,
    Path(payment_provider): Path<PaymentProvider>,
    Json(body): Json<GenericPayBody>,
) -> Result<impl IntoResponse, SolariError> {
    let request = match payment_provider {
        PaymentProvider::Vipps => PayRequest::Vipps(VippsPayRequest {
            amount: body.amount,
            return_url: body.return_url,
        }),
        PaymentProvider::Stripe => {
            if body.return_url.is_some() {
                return Err(SolariError::ApiError(
                    StatusCode::BAD_REQUEST,
                    "'return_url' is only supported for provider 'vipps'".to_string(),
                ));
            }

            PayRequest::Stripe(StripePayRequest {
                amount: body.amount,
                currency: body.currency,
                description: body.description,
            })
        }
    };

    let payment = state.payment_service.pay(request).await?;

    if let Some(handler) = &state.handlers.on_pay {
        handler(payment.clone()).await;
    }

    Ok((StatusCode::CREATED, Json(payment)))
}

async fn status(
    State(state): State<AppState>,
    Query(query): Query<GenericStatusQuery>,
) -> Result<impl IntoResponse, SolariError> {
    let reference = query.reference.trim();
    if reference.is_empty() {
        return Err(SolariError::ApiError(
            StatusCode::BAD_REQUEST,
            "missing or empty 'reference' query parameter".to_string(),
        ));
    }

    match query.provider {
        PaymentProvider::Vipps => {
            let result = state
                .payment_service
                .vipps_get_payment_status(reference)
                .await?;

            Ok(Json(GenericPaymentResponse {
                provider: PaymentProvider::Vipps,
                status: result.status.to_string(),
                paid: 0,
                currency: result.currency,
                reference: result.reference,
                redirect_url: None,
                return_url: None,
                raw_status: Some(result.raw_status),
            }))
        }
        PaymentProvider::Stripe => Err(SolariError::ApiError(
            StatusCode::BAD_REQUEST,
            "status endpoint is currently only supported for provider 'vipps'".to_string(),
        )),
    }
}

#[cfg(feature = "vipps")]
async fn vipps_webhook(
    State(state): State<AppState>,
    Json(body): Json<VippsWebhookPayload>,
) -> Result<impl IntoResponse, SolariError> {
    info!("Received webhook request from vipps: {:?}", body);
    if let Some(handler) = &state.handlers.on_vipps_webhook {
        handler(WebhookEvent {
            provider: PaymentProvider::Vipps,
            payload: body,
        })
        .await;
    }

    Ok(StatusCode::ACCEPTED)
}

#[cfg(feature = "stripe")]
async fn stripe_webhook(
    State(state): State<AppState>,
    Json(body): Json<StripeWebhookPayload>,
) -> Result<impl IntoResponse, SolariError> {
    info!("Received webhook request from stripe: {:?}", body);
    if let Some(handler) = &state.handlers.on_stripe_webhook {
        handler(WebhookEvent {
            provider: PaymentProvider::Stripe,
            payload: body,
        })
        .await;
    }

    Ok(StatusCode::ACCEPTED)
}
