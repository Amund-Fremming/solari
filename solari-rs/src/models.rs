use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct PaymentProviderResponse {
    pub provider: PaymentType,
    pub status: PaymentStatus,
    pub paid: u32,
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
    pub return_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PaymentType {
    Vipps,
    ApplePay,
    Stripe,
}

impl fmt::Display for PaymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PaymentType::Vipps => "vipps",
            PaymentType::ApplePay => "apple_pay",
            PaymentType::Stripe => "stripe",
        };

        f.write_str(name)
    }
}

#[cfg(feature = "api")]
use std::sync::Arc;

#[cfg(feature = "api")]
use crate::SolariPaymentService;

#[cfg(feature = "api")]
use crate::webhooks::SolariHandlers;

#[cfg(feature = "api")]
#[derive(Clone)]
pub struct AppState {
    pub payment_service: Arc<SolariPaymentService>,
    pub handlers: Arc<SolariHandlers>,
}

#[cfg(feature = "api")]
impl AppState {
    pub fn new(payment_module: SolariPaymentService) -> Self {
        Self {
            payment_service: Arc::new(payment_module),
            handlers: Arc::new(SolariHandlers::default()),
        }
    }

    pub fn with_handlers(payment_module: SolariPaymentService, handlers: SolariHandlers) -> Self {
        Self {
            payment_service: Arc::new(payment_module),
            handlers: Arc::new(handlers),
        }
    }
}
