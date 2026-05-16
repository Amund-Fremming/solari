#[cfg(feature = "api")]
use std::sync::Arc;

#[cfg(feature = "api")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "api")]
use solari_core::SolariPaymentService;

#[cfg(feature = "api")]
#[derive(Clone)]
pub struct AppState {
    pub payment_service: Arc<SolariPaymentService>,
}

#[cfg(feature = "api")]
impl AppState {
    pub fn new(payment_module: SolariPaymentService) -> Self {
        Self {
            payment_service: Arc::new(payment_module),
        }
    }
}

#[cfg(feature = "api")]
#[derive(Debug, Deserialize)]
pub struct VippsPayBody {
    pub amount: u32,
    pub return_url: Option<String>,
}

#[cfg(feature = "api")]
#[derive(Debug, Deserialize)]
pub struct VippsCreatePaymentBody {
    pub amount: u32,
    pub return_url: Option<String>,
}

#[cfg(feature = "api")]
#[derive(Debug, Serialize)]
pub struct VippsPayResponse {
    pub provider: String,
    pub status: String,
    pub paid: u32,
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
    pub return_url: Option<String>,
}

#[cfg(feature = "api")]
#[derive(Debug, Serialize)]
pub struct VippsTokenResponseBody {
    pub access_token: String,
    pub expires_at: u64,
}

#[cfg(feature = "api")]
#[derive(Debug, Serialize)]
pub struct VippsCreatePaymentResponseBody {
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
}

#[cfg(feature = "api")]
#[derive(Debug, Deserialize)]
pub struct VippsStatusQuery {
    pub reference: String,
}

#[cfg(feature = "api")]
#[derive(Debug, Serialize)]
pub struct VippsStatusResponseBody {
    pub reference: Option<String>,
    pub raw_status: String,
    pub status: String,
}
