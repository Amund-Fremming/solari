use async_trait::async_trait;
use serde::Deserialize;
use tracing::{error, info};

use crate::{
    adapters::stripe::models::{StripeConfig, StripePaymentRequest, StripePaymentResult},
    traits::PaymentAdapter,
    PaymentProvider, PaymentResponse, PaymentStatus, SolariError,
};

#[derive(Debug, Deserialize)]
struct StripeApiPaymentResponse {
    id: String,
    status: String,
    amount: u32,
    currency: String,
}

#[derive(Debug)]
pub struct StripeAdapter {
    client: reqwest::Client,
    config: StripeConfig,
}

impl StripeAdapter {
    #[cfg(feature = "stripe")]
    pub fn new(client: reqwest::Client, config: StripeConfig) -> Self {
        Self { client, config }
    }

    pub async fn create_payment(
        &self,
        request: StripePaymentRequest,
    ) -> Result<StripePaymentResult, SolariError> {
        if request.amount == 0 {
            error!("Cannot create empty payment");
            return Err(SolariError::InvalidAmount(request.amount));
        }

        let endpoint = format!(
            "{}/v1/payment_intents",
            self.config.api_base_url.trim_end_matches('/')
        );

        info!(
            "Stripe create_payment_intent started: amount={} currency={}",
            request.amount, request.currency
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

        form_fields.push(("confirmation_method".to_string(), "automatic".to_string()));
        form_fields.push(("payment_method_types[]".to_string(), "card".to_string()));

        let encoded_form = serde_urlencoded::to_string(&form_fields).map_err(|err| {
            SolariError::RequestFailed(format!("failed to encode stripe form body: {err}"))
        })?;

        let mut request = self
            .client
            .post(endpoint)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.secret_key),
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(encoded_form);

        if let Some(account_id) = self.config.account_id.as_ref() {
            request = request.header("Stripe-Account", account_id);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!("Stripe create payment failed: {}", body);
            return Err(SolariError::RequestFailed(body));
        }

        let payload: StripeApiPaymentResponse = response.json().await?;

        info!(
            "Stripe create_payment_intent succeeded: intent_id={} amount={} status={}",
            payload.id, payload.amount, payload.status
        );

        Ok(StripePaymentResult {
            id: payload.id,
            status: payload.status,
            amount: payload.amount,
            currency: payload.currency,
        })
    }
}

#[async_trait]
impl PaymentAdapter for StripeAdapter {
    async fn pay(&self, amount: u32) -> Result<PaymentResponse, SolariError> {
        let result = self
            .create_payment(StripePaymentRequest {
                amount,
                currency: "nok".to_string(),
                description: Some("Solari card payment".to_string()),
            })
            .await?;

        info!(
            "💵 Stripe payment intent created: id={}, amount={} status={}",
            result.id, result.amount, result.status
        );

        Ok(PaymentResponse {
            provider: PaymentProvider::Stripe,
            status: PaymentStatus::Pending,
            paid: 0,
            reference: Some(result.id),
            redirect_url: None,
            return_url: None,
        })
    }
}
