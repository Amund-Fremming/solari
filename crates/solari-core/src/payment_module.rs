use crate::modules::{
    apple_pay::models::{ApplePayConfig, ApplePayProvider},
    stripe::models::{StripeConfig, StripeProvider},
    vipps::models::{VippsConfig, VippsProvider},
};

#[derive(Debug)]
pub struct PaymentModule {
    pub vipps: Option<VippsProvider>,
    pub apple_pay: Option<ApplePayProvider>,
    pub stripe: Option<StripeProvider>,
}

impl PaymentModule {
    pub fn new() -> Self {
        PaymentModule {
            vipps: None,
            apple_pay: None,
            stripe: None,
        }
    }

    pub fn vipps(&mut self, _config: VippsConfig) -> &mut Self {
        // TODO - create instance with config, then set to some
        self.vipps = Some(VippsProvider);
        self
    }

    pub fn apple_pay(&mut self, _config: ApplePayConfig) -> &mut Self {
        // TODO - create instance with config, then set to some
        self.apple_pay = Some(ApplePayProvider);
        self
    }

    pub fn stripe(&mut self, _config: StripeConfig) -> &mut Self {
        // TODO - create instance with config, then set to some
        self.stripe = Some(StripeProvider);
        self
    }
}
