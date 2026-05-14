use crate::core::{PaymentProviderError, PaymentProviderResponse};

pub trait PaymentProvider {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError>;
}
