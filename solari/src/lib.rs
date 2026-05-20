mod adapters;
mod error;
mod models;
mod payment_module;
mod traits;

pub mod storage;
pub mod webhooks;

pub use error::PaymentProviderError;
pub use models::{PaymentProviderResponse, PaymentStatus, PaymentType};
#[cfg(feature = "stripe")]
pub use models::{StripePayRequest, StripePaymentFlowType, StripePaymentIntentResponse};
#[cfg(feature = "vipps")]
pub use models::{
    VippsCreatePaymentResult, VippsPayRequest, VippsPaymentStatusResult, VippsTokenResponse,
};
pub use payment_module::{PayRequest, SolariPaymentService, StripeConfig, VippsConfig};

pub use webhooks::{OnPayFn, SolariHandlers, WebhookEvent};
