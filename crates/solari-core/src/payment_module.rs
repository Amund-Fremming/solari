use crate::{
    core::{PaymentProviderResponse, PaymentType},
    error::PaymentProviderError,
    modules::{
        apple_pay::models::{ApplePayConfig, ApplePayProvider},
        stripe::models::{StripeConfig, StripeProvider},
        vipps::{models::VippsConfig, provider::VippsProvider},
    },
    traits::PaymentProvider,
};
use std::{collections::HashMap, time::Duration};

pub struct PaymentModule {
    client: reqwest::Client,
    providers: HashMap<PaymentType, Box<dyn PaymentProvider + Send + Sync>>,
}

impl PaymentModule {
    pub fn new() -> Result<Self, PaymentProviderError> {
        Ok(PaymentModule {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .connect_timeout(Duration::from_secs(10))
                .pool_idle_timeout(Duration::from_secs(90))
                .pool_max_idle_per_host(8)
                .tcp_keepalive(Duration::from_secs(60))
                .user_agent("solari-core/0.1")
                .build()?,
            providers: HashMap::new(),
        })
    }

    pub fn vipps(&mut self, config: VippsConfig) -> &mut Self {
        let provider = VippsProvider::new(self.client.clone(), config);

        self.providers
            .insert(PaymentType::Vipps, Box::new(provider));
        self
    }

    pub fn apple_pay(&mut self, config: ApplePayConfig) -> &mut Self {
        let provider = ApplePayProvider::new(self.client.clone(), config);

        self.providers
            .insert(PaymentType::ApplePay, Box::new(provider));
        self
    }

    pub fn stripe(&mut self, config: StripeConfig) -> &mut Self {
        let provider = StripeProvider::new(self.client.clone(), config);

        self.providers
            .insert(PaymentType::Stripe, Box::new(provider));
        self
    }

    pub async fn pay(
        &self,
        provider: PaymentType,
        amount: u32,
    ) -> Result<PaymentProviderResponse, PaymentProviderError> {
        self.providers
            .get(&provider)
            .ok_or(PaymentProviderError::NotConfigured(provider))?
            .pay(amount)
            .await
    }
}
