use crate::{
    core::{PaymentProviderError, PaymentProviderResponse, PaymentStatus},
    traits::PaymentProvider,
};

#[derive(Debug)]
pub struct VippsConfig {
    //
}

impl VippsConfig {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct VippsProvider;

impl VippsProvider {
    pub fn new(_config: VippsConfig) -> Self {
        Self
    }
}

impl PaymentProvider for VippsProvider {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        // TODO!
        Ok(PaymentProviderResponse {
            status: PaymentStatus::Completed,
            paid: amount,
        })
    }
}
