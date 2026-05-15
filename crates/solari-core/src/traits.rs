use crate::{core::PaymentProviderResponse, error::PaymentProviderError};
use async_trait::async_trait;

#[async_trait]
pub trait PaymentProvider: Send + Sync {
    async fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError>;
}
