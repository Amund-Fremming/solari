use crate::{
    core::{PaymentProviderError, PaymentProviderResponse, PaymentStatus},
    traits::PaymentProvider,
};

#[derive(Debug)]
pub struct ApplePayConfig {
    //
}

impl ApplePayConfig {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ApplePayProvider;

impl ApplePayProvider {
    pub fn new(_config: ApplePayConfig) -> Self {
        Self
    }
}

impl PaymentProvider for ApplePayProvider {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        // TODO!
        Ok(PaymentProviderResponse {
            status: PaymentStatus::Completed,
            paid: amount,
        })
    }
}
