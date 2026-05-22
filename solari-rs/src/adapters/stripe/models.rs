use crate::{
    error::PaymentProviderError,
    models::{PaymentProviderResponse, PaymentStatus, PaymentType},
    traits::PaymentProvider,
};
use async_trait::async_trait;
use serde::Deserialize;
use tracing::{error, info};

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

#[derive(Debug)]
pub struct StripeProvider {
    client: reqwest::Client,
    config: StripeConfig,
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
struct StripeErrorEnvelope {
    error: Option<StripeErrorObject>,
}

#[derive(Debug, Deserialize)]
struct StripeErrorObject {
    message: Option<String>,
}

impl StripeProvider {
    pub fn new(client: reqwest::Client, config: StripeConfig) -> Self {
        Self { client, config }
    }

    pub async fn create_payment_intent(
        &self,
        request: StripeCreatePaymentIntentRequest,
    ) -> Result<StripePaymentIntentResult, PaymentProviderError> {
        let flow_label = match request.flow {
            StripePaymentFlow::Card => "card",
            StripePaymentFlow::ApplePay => "apple_pay",
        };

        if request.amount == 0 {
            error!(
                "Stripe create_payment_intent rejected invalid amount: flow={} amount={}",
                flow_label, request.amount
            );
            return Err(PaymentProviderError::InvalidAmount(request.amount));
        }

        let endpoint = format!(
            "{}/v1/payment_intents",
            self.config.api_base_url.trim_end_matches('/')
        );

        info!(
            "Stripe create_payment_intent started: flow={} amount={} currency={}",
            flow_label, request.amount, request.currency
        );

        let mut form_fields = vec![
            ("amount".to_string(), request.amount.to_string()),
            ("currency".to_string(), request.currency.to_lowercase()),
            ("confirm".to_string(), "false".to_string()),
        ];

        if let Some(description) = request.description {
            let trimmed = description.trim();
            if !trimmed.is_empty() {
                form_fields.push(("description".to_string(), trimmed.to_string()));
            }
        }

        match request.flow {
            StripePaymentFlow::Card => {
                form_fields.push(("confirmation_method".to_string(), "automatic".to_string()));
                form_fields.push(("payment_method_types[]".to_string(), "card".to_string()));
            }
            StripePaymentFlow::ApplePay => {
                form_fields.push(("confirmation_method".to_string(), "automatic".to_string()));
                form_fields.push(("payment_method_types[]".to_string(), "card".to_string()));
            }
        }

        let encoded_form = serde_urlencoded::to_string(&form_fields).map_err(|err| {
            PaymentProviderError::RequestFailed(format!("failed to encode stripe form body: {err}"))
        })?;

        let mut req = self
            .client
            .post(endpoint)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.secret_key),
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(encoded_form);

        if let Some(account_id) = self.config.account_id.as_ref() {
            req = req.header("Stripe-Account", account_id);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            let message = serde_json::from_str::<StripeErrorEnvelope>(&body)
                .ok()
                .and_then(|payload| payload.error)
                .and_then(|payload| payload.message)
                .unwrap_or_else(|| {
                    if body.trim().is_empty() {
                        format!("stripe returned {}", status)
                    } else {
                        body
                    }
                });

            error!(
                "Stripe create_payment_intent failed: flow={} status={} message={}",
                flow_label, status, message
            );

            return Err(PaymentProviderError::RequestFailed(message));
        }

        let payload: StripePaymentIntentResponse = resp.json().await?;
        let client_secret = payload.client_secret.ok_or_else(|| {
            PaymentProviderError::RequestFailed(
                "stripe response missing client_secret for payment intent".to_string(),
            )
        })?;

        info!(
            "Stripe create_payment_intent succeeded: flow={} intent_id={} amount={} status={}",
            flow_label, payload.id, payload.amount, payload.status
        );

        Ok(StripePaymentIntentResult {
            id: payload.id,
            client_secret,
            status: payload.status,
            amount: payload.amount,
            currency: payload.currency,
            publishable_key: self.config.publishable_key.clone(),
            account_id: self.config.account_id.clone(),
        })
    }
}

#[async_trait]
impl PaymentProvider for StripeProvider {
    async fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        let intent = self
            .create_payment_intent(StripeCreatePaymentIntentRequest {
                amount,
                currency: "nok".to_string(),
                description: Some("Solari card payment".to_string()),
                flow: StripePaymentFlow::Card,
            })
            .await?;

        info!(
            "💵 Stripe payment intent created: intent_id={}, amount={} status={}",
            intent.id, intent.amount, intent.status
        );

        Ok(PaymentProviderResponse {
            provider: PaymentType::Stripe,
            status: PaymentStatus::Pending,
            paid: 0,
            reference: Some(intent.id),
            redirect_url: None,
            return_url: None,
        })
    }
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
