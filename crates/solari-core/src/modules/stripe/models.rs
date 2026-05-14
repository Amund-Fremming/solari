use crate::{
    core::{PaymentProviderError, PaymentProviderResponse, PaymentStatus},
    traits::PaymentProvider,
};

#[derive(Debug)]
pub struct StripeConfig {
    //
}

impl StripeConfig {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct StripeProvider;

impl StripeProvider {
    pub fn new(_config: StripeConfig) -> Self {
        Self
    }
}

impl PaymentProvider for StripeProvider {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        // TODO!
        Ok(PaymentProviderResponse {
            status: PaymentStatus::Completed,
            paid: amount,
        })
    }
}
