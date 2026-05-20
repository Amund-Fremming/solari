pub mod core;

#[cfg(feature = "vipps")]
pub mod vipps;

#[cfg(feature = "stripe")]
pub mod stripe;

#[cfg(any(feature = "vipps", feature = "stripe"))]
pub use core::AppState;

pub use core::{
    BoxFuture, OnPayFn, PaymentProviderResponse, PaymentStatus, PaymentType, SolariHandlers,
    WebhookEvent,
};

#[cfg(feature = "vipps")]
pub use vipps::{VippsWebhookFn, VippsWebhookPayload};

#[cfg(feature = "stripe")]
pub use stripe::{StripeWebhookFn, StripeWebhookPayload};
