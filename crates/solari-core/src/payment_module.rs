use crate::{
    core::{PaymentProviderError, PaymentProviderResponse, PaymentType},
    modules::{
        apple_pay::models::{ApplePayConfig, ApplePayProvider},
        stripe::models::{StripeConfig, StripeProvider},
        vipps::models::{VippsConfig, VippsProvider},
    },
    traits::PaymentProvider,
};
use std::collections::HashMap;

pub struct PaymentModule {
    providers: HashMap<PaymentType, Box<dyn PaymentProvider + Send + Sync>>,
}

impl PaymentModule {
    pub fn new() -> Self {
        PaymentModule {
            providers: HashMap::new(),
        }
    }

    pub fn vipps(&mut self, config: VippsConfig) -> &mut Self {
        // TODO - create instance with config, then set to some
        let provider = VippsProvider::new(config);

        self.providers
            .insert(PaymentType::Vipps, Box::new(provider));
        self
    }

    pub fn apple_pay(&mut self, config: ApplePayConfig) -> &mut Self {
        // TODO - create instance with config, then set to some
        let provider = ApplePayProvider::new(config);

        self.providers
            .insert(PaymentType::ApplePay, Box::new(provider));
        self
    }

    pub fn stripe(&mut self, config: StripeConfig) -> &mut Self {
        // TODO - create instance with config, then set to some
        let provider = StripeProvider::new(config);

        self.providers
            .insert(PaymentType::Stripe, Box::new(provider));
        self
    }

    pub fn pay(
        &self,
        provider: PaymentType,
        amount: u32,
    ) -> Result<PaymentProviderResponse, PaymentProviderError> {
        self.providers
            .get(&provider)
            .ok_or(PaymentProviderError::NotConfigured(provider))?
            .pay(amount)
    }
}
