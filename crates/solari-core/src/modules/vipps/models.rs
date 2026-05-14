use crate::{
    core::{PaymentProviderError, PaymentProviderResponse, PaymentStatus},
    traits::PaymentProvider,
};

#[derive(Debug)]
pub struct VippsConfig {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub subscription_key: String,
    pub merchant_serial_number: String,
}

impl VippsConfig {
    pub fn new(
        base_url: String,
        client_id: String,
        client_secret: String,
        subscription_key: String,
        merchant_serial_number: String,
    ) -> Self {
        Self {
            base_url,
            client_id,
            client_secret,
            subscription_key,
            merchant_serial_number,
        }
    }
}

#[derive(Debug)]
pub struct VippsProvider {
    config: VippsConfig,
}

impl VippsProvider {
    pub fn new(config: VippsConfig) -> Self {
        Self { config }
    }
}

impl PaymentProvider for VippsProvider {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        let _base_url = &self.config.base_url;

        // TODO!
        Ok(PaymentProviderResponse {
            status: PaymentStatus::Completed,
            paid: amount,
        })
    }
}
