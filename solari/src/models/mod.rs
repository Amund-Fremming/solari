pub mod core;

pub use core::{PaymentProviderResponse, PaymentStatus, PaymentType};

#[cfg(feature = "api")]
pub use core::AppState;

#[cfg(feature = "stripe")]
pub mod stripe;
#[cfg(feature = "vipps")]
pub mod vipps;

#[cfg(feature = "vipps")]
pub use vipps::{
    VippsCreatePaymentResult, VippsPayRequest, VippsPaymentStatusResult, VippsTokenResponse,
};

#[cfg(feature = "stripe")]
pub use stripe::{StripePayRequest, StripePaymentFlowType, StripePaymentIntentResponse};
