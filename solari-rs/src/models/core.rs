use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[cfg(feature = "vipps")]
use crate::models::VippsWebhookFn;

#[cfg(feature = "stripe")]
use crate::models::StripeWebhookFn;

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

pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub struct WebhookEvent<P> {
    pub provider: PaymentType,
    pub payload: P,
}

pub type OnPayFn = Arc<dyn Fn(PaymentProviderResponse) -> BoxFuture + Send + Sync>;

#[derive(Default)]
pub struct SolariHandlers {
    pub on_pay: Option<OnPayFn>,
    #[cfg(feature = "vipps")]
    pub on_vipps_webhook: Option<VippsWebhookFn>,
    #[cfg(feature = "stripe")]
    pub on_stripe_webhook: Option<StripeWebhookFn>,
}

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
