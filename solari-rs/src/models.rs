use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct GenericPayBody {
    pub amount: u32,
    pub return_url: Option<String>,
    pub currency: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenericStatusQuery {
    pub provider: PaymentProvider,
    pub reference: String,
}

#[derive(Debug, Serialize)]
pub struct GenericPaymentResponse {
    pub provider: PaymentProvider,
    pub status: String,
    pub paid: u32,
    pub currency: Option<String>,
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
    pub return_url: Option<String>,
    pub raw_status: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

impl ToString for PaymentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => "pending".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Failed => "failed".to_string(),
            Self::Cancelled => "cancelled".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentResponse {
    pub provider: PaymentProvider,
    pub status: PaymentStatus,
    pub paid: u32,
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
    pub return_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum PaymentProvider {
    Vipps,
    Stripe,
}

impl fmt::Display for PaymentProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PaymentProvider::Vipps => "vipps",
            PaymentProvider::Stripe => "stripe",
        };

        f.write_str(name)
    }
}

pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub struct WebhookEvent<P> {
    pub provider: PaymentProvider,
    pub payload: P,
}

pub type OnPayFn = Arc<dyn Fn(PaymentResponse) -> BoxFuture + Send + Sync>;

#[derive(Default)]
pub struct SolariHandlers {
    pub on_pay: Option<OnPayFn>,
    #[cfg(feature = "vipps")]
    pub on_vipps_webhook: Option<VippsWebhookFn>,
    #[cfg(feature = "stripe")]
    pub on_stripe_webhook: Option<StripeWebhookFn>,
}

use serde::{Deserialize, Serialize};

#[cfg(feature = "stripe")]
use crate::adapters::stripe::models::StripeWebhookFn;
#[cfg(feature = "vipps")]
use crate::adapters::vipps::models::VippsWebhookFn;
#[cfg(any(feature = "vipps", feature = "stripe"))]
use crate::SolariPaymentService;

#[cfg(any(feature = "vipps", feature = "stripe"))]
#[derive(Clone)]
pub struct AppState {
    pub payment_service: Arc<SolariPaymentService>,
    pub handlers: Arc<SolariHandlers>,
}

#[cfg(any(feature = "vipps", feature = "stripe"))]
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
