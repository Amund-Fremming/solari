use std::sync::Arc;

use serde::Deserialize;

use crate::models::{BoxFuture, WebhookEvent};

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

pub type VippsWebhookFn = Arc<dyn Fn(WebhookEvent<VippsWebhookPayload>) -> BoxFuture + Send + Sync>;