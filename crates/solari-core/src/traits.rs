use crate::core::{PaymentProviderError, PaymentProviderResponse};

pub trait PaymentProvider {
    fn pay(price: u32) -> Result<PaymentProviderResponse, PaymentProviderError>;
}
