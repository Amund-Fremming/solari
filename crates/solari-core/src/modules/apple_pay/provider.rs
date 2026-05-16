use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use tracing::info;

use crate::{
    core::{PaymentProviderResponse, PaymentStatus, PaymentType},
    error::PaymentProviderError,
    modules::apple_pay::models::ApplePayConfig,
    traits::PaymentProvider,
};

#[derive(Debug)]
pub struct ApplePayProvider {
    client: reqwest::Client,
    config: ApplePayConfig,
}

impl ApplePayProvider {
    pub fn new(client: reqwest::Client, config: ApplePayConfig) -> Self {
        Self { client, config }
    }

    fn now() -> Result<u64, PaymentProviderError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())?;
        Ok(now)
    }

    fn validate_config(&self) -> Result<(), PaymentProviderError> {
        let mut missing: Vec<&str> = Vec::new();

        if self.config.merchant_id.trim().is_empty() {
            missing.push("merchant_id");
        }
        if self.config.merchant_display_name.trim().is_empty() {
            missing.push("merchant_display_name");
        }
        if self.config.initiative.trim().is_empty() {
            missing.push("initiative");
        }
        if self.config.initiative_context.trim().is_empty() {
            missing.push("initiative_context");
        }
        if self.config.merchant_validation_url.trim().is_empty() {
            missing.push("merchant_validation_url");
        }
        if self.config.payment_processing_cert_pem.trim().is_empty() {
            missing.push("payment_processing_cert_pem");
        }
        if self.config.payment_processing_key_pem.trim().is_empty() {
            missing.push("payment_processing_key_pem");
        }

        if !missing.is_empty() {
            return Err(PaymentProviderError::RequestFailed(format!(
                "apple_pay is missing required config values: {}",
                missing.join(", ")
            )));
        }

        Ok(())
    }
}

#[async_trait]
impl PaymentProvider for ApplePayProvider {
    async fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        if amount == 0 {
            return Err(PaymentProviderError::InvalidAmount(amount));
        }

        self.validate_config()?;

        // Keep the client in use for future network-backed Apple Pay flow.
        let _client = &self.client;

        let now = Self::now()?;
        let reference = format!("applepay-{now}-{amount}");

        info!("💵 Apple Pay payment initialized: {amount} NOK");

        Ok(PaymentProviderResponse {
            provider: PaymentType::ApplePay,
            status: PaymentStatus::Pending,
            paid: 0,
            reference: Some(reference),
            redirect_url: None,
            return_url: None,
        })
    }
}
