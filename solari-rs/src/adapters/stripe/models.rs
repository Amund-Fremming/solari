use serde::Deserialize;

#[cfg(feature = "stripe")]
use serde_json::Value;

#[cfg(feature = "stripe")]
use std::sync::Arc;

#[cfg(feature = "stripe")]
use crate::models::{BoxFuture, WebhookEvent};

#[derive(Debug)]
pub struct StripeConfig {
    pub api_base_url: String,
    pub secret_key: String,
    pub publishable_key: String,
    pub webhook_secret: String,
    pub account_id: Option<String>,
}

impl StripeConfig {
    pub fn new(
        api_base_url: String,
        secret_key: String,
        publishable_key: String,
        webhook_secret: String,
        account_id: Option<String>,
    ) -> Self {
        Self {
            api_base_url,
            secret_key,
            publishable_key,
            webhook_secret,
            account_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StripePaymentFlow {
    Card,
    ApplePay,
}

#[derive(Debug, Clone)]
pub struct StripeCreatePaymentIntentRequest {
    pub amount: u32,
    pub currency: String,
    pub description: Option<String>,
    pub flow: StripePaymentFlow,
}

#[derive(Debug, Clone)]
pub struct StripePaymentIntentResult {
    pub id: String,
    pub client_secret: String,
    pub status: String,
    pub amount: u32,
    pub currency: String,
    pub publishable_key: String,
    pub account_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripePaymentIntentResponse {
    id: String,
    client_secret: Option<String>,
    status: String,
    amount: u32,
    currency: String,
}

#[derive(Debug, Deserialize)]
pub struct StripeErrorEnvelope {
    error: Option<StripeErrorObject>,
}

#[derive(Debug, Deserialize)]
struct StripeErrorObject {
    message: Option<String>,
}

#[cfg(feature = "stripe")]
#[derive(Debug, Clone, Deserialize)]
pub struct StripeWebhookPayload {
    pub id: String,
    pub object: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created: i64,
    pub data: StripeWebhookData,
}

#[cfg(feature = "stripe")]
#[derive(Debug, Clone, Deserialize)]
pub struct StripeWebhookData {
    pub object: Value,
}

#[cfg(feature = "stripe")]
pub type StripeWebhookFn =
    Arc<dyn Fn(WebhookEvent<StripeWebhookPayload>) -> BoxFuture + Send + Sync>;
