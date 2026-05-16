use crate::{
    core::{PaymentProviderResponse, PaymentStatus, PaymentType},
    error::PaymentProviderError,
    traits::PaymentProvider,
};
use async_trait::async_trait;
use tracing::info;

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

impl StripeProvider {
    pub fn new(client: reqwest::Client, config: StripeConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl PaymentProvider for StripeProvider {
    async fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        let _client = &self.client;
        let _api_base_url = &self.config.api_base_url;

        info!("💵 Stripe payment completed: {amount} paid");

        Ok(PaymentProviderResponse {
            provider: PaymentType::Stripe,
            status: PaymentStatus::Completed,
            paid: amount,
            reference: None,
            redirect_url: None,
            return_url: None,
        })
    }
}
