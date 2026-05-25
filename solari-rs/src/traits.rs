use crate::{error::SolariError, models::PaymentResponse};
use async_trait::async_trait;

#[async_trait]
pub trait PaymentAdapter: Send + Sync {
    async fn pay(&self, amount: u32) -> Result<PaymentResponse, SolariError>;
}
