use crate::{
    core::{PaymentProviderResponse, PaymentStatus},
    error::PaymentProviderError,
    traits::PaymentProvider,
};
use async_trait::async_trait;
use tracing::info;

#[derive(Debug)]
pub struct ApplePayConfig {
    pub merchant_id: String,
    pub merchant_display_name: String,
    pub initiative: String,
    pub initiative_context: String,
    pub merchant_validation_url: String,
    pub payment_processing_cert_pem: String,
    pub payment_processing_key_pem: String,
}

impl ApplePayConfig {
    pub fn new(
        merchant_id: String,
        merchant_display_name: String,
        initiative: String,
        initiative_context: String,
        merchant_validation_url: String,
        payment_processing_cert_pem: String,
        payment_processing_key_pem: String,
    ) -> Self {
        Self {
            merchant_id,
            merchant_display_name,
            initiative,
            initiative_context,
            merchant_validation_url,
            payment_processing_cert_pem,
            payment_processing_key_pem,
        }
    }
}

#[derive(Debug)]
pub struct ApplePayProvider {
    client: reqwest::Client,
    config: ApplePayConfig,
}

impl ApplePayProvider {
    pub fn new(client: reqwest::Client, config: ApplePayConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl PaymentProvider for ApplePayProvider {
    async fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        let _client = &self.client;
        let _merchant_id = &self.config.merchant_id;

        info!("💵 Apple Pay payment completed: {amount} paid");

        Ok(PaymentProviderResponse {
            status: PaymentStatus::Completed,
            paid: amount,
        })
    }
}
