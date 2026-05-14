use crate::{
    core::{PaymentProviderError, PaymentProviderResponse, PaymentStatus},
    traits::PaymentProvider,
};

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
    config: StripeConfig,
}

impl StripeProvider {
    pub fn new(config: StripeConfig) -> Self {
        Self { config }
    }
}

impl PaymentProvider for StripeProvider {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        let _api_base_url = &self.config.api_base_url;

        // TODO!
        Ok(PaymentProviderResponse {
            status: PaymentStatus::Completed,
            paid: amount,
        })
    }
}
