use crate::core::{PaymentProviderError, PaymentProviderResponse};

pub trait PaymentProvider: Send + Sync {
    fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError>;
}
