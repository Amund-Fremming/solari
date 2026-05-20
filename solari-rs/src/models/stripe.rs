use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::models::{BoxFuture, WebhookEvent};

#[derive(Debug, Clone, Deserialize)]
pub struct StripeWebhookPayload {
    pub id: String,
    pub object: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created: i64,
    pub data: StripeWebhookData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeWebhookData {
    pub object: Value,
}

pub type StripeWebhookFn =
    Arc<dyn Fn(WebhookEvent<StripeWebhookPayload>) -> BoxFuture + Send + Sync>;