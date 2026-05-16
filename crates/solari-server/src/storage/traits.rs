use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPayment {
    pub reference: String,
    pub status: String,
}

pub trait PaymentStore: Send + Sync {
    fn upsert_payment(&self, payment: StoredPayment) -> Result<(), String>;
    fn get_payment(&self, reference: &str) -> Result<Option<StoredPayment>, String>;
}
