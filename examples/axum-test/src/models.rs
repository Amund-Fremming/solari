use std::sync::Arc;

use serde::Serialize;
use solari_core::SolariPaymentService;
use tokio::sync::RwLock;

pub const VIPPS_PAY_AMOUNT_NOK: u32 = 67;

#[derive(Clone)]
pub struct AppState {
    pub payment_module: Arc<SolariPaymentService>,
    pub payment_state: Arc<RwLock<PaymentState>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSnapshot {
    pub provider: String,
    pub status: String,
    pub requested_amount: u32,
    pub paid_amount: u32,
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
    pub return_url: Option<String>,
    pub attempts: u64,
    pub updated_from: String,
    pub last_error: Option<String>,
    pub raw_status: Option<String>,
}

impl Default for PaymentSnapshot {
    fn default() -> Self {
        Self {
            provider: "vipps".to_string(),
            status: "idle".to_string(),
            requested_amount: 0,
            paid_amount: 0,
            reference: None,
            redirect_url: None,
            return_url: None,
            attempts: 0,
            updated_from: "startup".to_string(),
            last_error: None,
            raw_status: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PaymentState {
    pub snapshot: PaymentSnapshot,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub ok: bool,
    pub payment: PaymentSnapshot,
}
