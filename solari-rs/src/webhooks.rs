use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::models::{PaymentProviderResponse, PaymentType};

pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub struct WebhookEvent<P> {
    pub provider: PaymentType,
    pub payload: P,
}

#[cfg(feature = "vipps")]
use serde::Deserialize;

#[cfg(feature = "vipps")]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsWebhookPayload {
    pub msn: String,
    pub reference: String,
    pub psp_reference: Option<String>,
    pub name: String,
    pub currency: String,
    pub amount: i64,
    pub timestamp: String,
    pub idempotency_key: Option<String>,
    pub success: bool,
}

#[cfg(feature = "vipps")]
pub type VippsWebhookFn =
    Arc<dyn Fn(WebhookEvent<VippsWebhookPayload>) -> BoxFuture + Send + Sync>;

#[cfg(feature = "stripe")]
use serde_json::Value;

#[cfg(feature = "stripe")]
use serde::Deserialize as StripeDeserialize;

#[cfg(feature = "stripe")]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct StripeWebhookPayload {
    pub id: String,
    pub object: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created: i64,
    pub data: StripeWebhookData,
}

#[cfg(feature = "stripe")]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct StripeWebhookData {
    pub object: Value,
}

#[cfg(feature = "stripe")]
pub type StripeWebhookFn =
    Arc<dyn Fn(WebhookEvent<StripeWebhookPayload>) -> BoxFuture + Send + Sync>;

pub type OnPayFn = Arc<dyn Fn(PaymentProviderResponse) -> BoxFuture + Send + Sync>;

#[derive(Default)]
pub struct SolariHandlers {
    pub on_pay: Option<OnPayFn>,
    #[cfg(feature = "vipps")]
    pub on_vipps_webhook: Option<VippsWebhookFn>,
    #[cfg(feature = "stripe")]
    pub on_stripe_webhook: Option<StripeWebhookFn>,
}
