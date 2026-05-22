use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::{ApiResult, SolariApiError},
    models::{AppState, PaymentType, WebhookEvent},
    PayRequest, PaymentProviderResponse, PaymentStatus, StripePayRequest, StripePaymentFlowType,
    StripePaymentIntentResponse, VippsPayRequest,
};

#[cfg(feature = "vipps")]
use crate::adapters::vipps::models::VippsWebhookPayload;

#[cfg(feature = "stripe")]
use crate::adapters::stripe::models::StripeWebhookPayload;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentProviderInput {
    Vipps,
    Stripe,
    ApplePay,
}

#[derive(Debug, Deserialize)]
pub struct GenericPayBody {
    pub provider: PaymentProviderInput,
    pub amount: u32,
    pub return_url: Option<String>,
    pub currency: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenericStatusQuery {
    pub provider: PaymentProviderInput,
    pub reference: String,
}

#[derive(Debug, Serialize)]
pub struct GenericPaymentResponse {
    pub provider: String,
    pub status: String,
    pub paid: u32,
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
    pub return_url: Option<String>,
    pub raw_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WebhookAck {
    pub ok: bool,
    pub provider: &'static str,
    pub received: bool,
}

#[derive(Debug, Deserialize)]
pub struct StripeIntentPayBody {
    pub amount: u32,
    pub currency: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StripePaymentIntentApiResponse {
    pub provider: String,
    pub flow: &'static str,
    pub status: String,
    pub amount: u32,
    pub currency: String,
    pub payment_intent_id: String,
    pub client_secret: String,
    pub publishable_key: String,
    pub account_id: Option<String>,
}

pub fn router() -> Router<AppState> {
    let r = Router::new()
        .route("/pay", post(pay))
        .route("/stripe/pay", post(stripe_pay))
        .route("/apple-pay/pay", post(apple_pay_pay))
        .route("/status", get(status));

    #[cfg(feature = "vipps")]
    let r = r.route("/webhooks/vipps", post(vipps_webhook));

    #[cfg(feature = "stripe")]
    let r = r.route("/webhooks/stripe", post(stripe_webhook));

    r
}

async fn pay(
    State(state): State<AppState>,
    Json(body): Json<GenericPayBody>,
) -> ApiResult<Json<GenericPaymentResponse>> {
    let request = match body.provider {
        PaymentProviderInput::Vipps => PayRequest::Vipps(VippsPayRequest {
            amount: body.amount,
            return_url: body.return_url,
        }),
        PaymentProviderInput::Stripe | PaymentProviderInput::ApplePay => {
            if body.return_url.is_some() {
                return Err(SolariApiError::BadRequest(
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

    Ok(Json(payment_response_from_provider(payment, None)))
}

async fn status(
    State(state): State<AppState>,
    Query(query): Query<GenericStatusQuery>,
) -> ApiResult<Json<GenericPaymentResponse>> {
    let reference = query.reference.trim();
    if reference.is_empty() {
        return Err(SolariApiError::BadRequest(
            "missing or empty 'reference' query parameter".to_string(),
        ));
    }

    match query.provider {
        PaymentProviderInput::Vipps => {
            let result = state
                .payment_service
                .vipps_get_payment_status(reference)
                .await?;

            Ok(Json(GenericPaymentResponse {
                provider: "vipps".to_string(),
                status: payment_status_to_str(result.status).to_string(),
                paid: 0,
                reference: result.reference,
                redirect_url: None,
                return_url: None,
                raw_status: Some(result.raw_status),
            }))
        }
        PaymentProviderInput::Stripe | PaymentProviderInput::ApplePay => {
            Err(SolariApiError::BadRequest(
                "status endpoint is currently only supported for provider 'vipps'".to_string(),
            ))
        }
    }
}

#[cfg(feature = "vipps")]
async fn vipps_webhook(
    State(state): State<AppState>,
    Json(body): Json<VippsWebhookPayload>,
) -> Json<WebhookAck> {
    if let Some(handler) = &state.handlers.on_vipps_webhook {
        handler(WebhookEvent {
            provider: PaymentType::Vipps,
            payload: body,
        })
        .await;
    }
    Json(WebhookAck { ok: true, provider: "vipps", received: true })
}

#[cfg(feature = "stripe")]
async fn stripe_webhook(
    State(state): State<AppState>,
    Json(body): Json<StripeWebhookPayload>,
) -> Json<WebhookAck> {
    if let Some(handler) = &state.handlers.on_stripe_webhook {
        handler(WebhookEvent {
            provider: PaymentType::Stripe,
            payload: body,
        })
        .await;
    }
    Json(WebhookAck { ok: true, provider: "stripe", received: true })
}

async fn stripe_pay(
    State(state): State<AppState>,
    Json(body): Json<StripeIntentPayBody>,
) -> ApiResult<Json<StripePaymentIntentApiResponse>> {
    let result = state
        .payment_service
        .stripe_create_card_payment_intent(StripePayRequest {
            amount: body.amount,
            currency: body.currency,
            description: body.description,
        })
        .await?;

    Ok(Json(stripe_intent_response(result)))
}

async fn apple_pay_pay(
    State(state): State<AppState>,
    Json(body): Json<StripeIntentPayBody>,
) -> ApiResult<Json<StripePaymentIntentApiResponse>> {
    let result = state
        .payment_service
        .stripe_create_apple_pay_payment_intent(StripePayRequest {
            amount: body.amount,
            currency: body.currency,
            description: body.description,
        })
        .await?;

    Ok(Json(stripe_intent_response(result)))
}

fn payment_response_from_provider(
    payment: PaymentProviderResponse,
    raw_status: Option<String>,
) -> GenericPaymentResponse {
    GenericPaymentResponse {
        provider: payment.provider.to_string(),
        status: payment_status_to_str(payment.status).to_string(),
        paid: payment.paid,
        reference: payment.reference,
        redirect_url: payment.redirect_url,
        return_url: payment.return_url,
        raw_status,
    }
}

fn payment_status_to_str(status: PaymentStatus) -> &'static str {
    match status {
        PaymentStatus::Pending => "pending",
        PaymentStatus::Completed => "completed",
        PaymentStatus::Failed => "failed",
        PaymentStatus::Cancelled => "cancelled",
    }
}

fn stripe_intent_response(intent: StripePaymentIntentResponse) -> StripePaymentIntentApiResponse {
    StripePaymentIntentApiResponse {
        provider: intent.provider.to_string(),
        flow: match intent.flow {
            StripePaymentFlowType::Card => "card",
            StripePaymentFlowType::ApplePay => "apple_pay",
        },
        status: intent.status,
        amount: intent.amount,
        currency: intent.currency,
        payment_intent_id: intent.payment_intent_id,
        client_secret: intent.client_secret,
        publishable_key: intent.publishable_key,
        account_id: intent.account_id,
    }
}
