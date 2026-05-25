mod adapters;
mod error;
#[cfg(any(feature = "vipps", feature = "stripe"))]
mod handlers;
mod models;
mod payment_module;
#[cfg(any(feature = "vipps", feature = "stripe"))]
mod solari_router;
mod traits;

pub use error::SolariError;
pub use models::{
    OnPayFn, PaymentProvider, PaymentResponse, PaymentStatus, SolariHandlers, WebhookEvent,
};
pub use payment_module::{
    PayRequest, SolariPaymentService, StripeConfig, StripePayRequest, StripePaymentFlowType,
    StripePaymentResponse, VippsConfig, VippsCreatePaymentResult, VippsPayRequest,
    VippsPaymentResult, VippsTokenResponse,
};
pub use traits::PaymentAdapter;

#[cfg(any(feature = "vipps", feature = "stripe"))]
pub use solari_router::{Solari, SolariApi, SolariRouter};

#[cfg(any(feature = "vipps", feature = "stripe"))]
pub use handlers::{app_router, solari_router};
#[cfg(feature = "vipps")]
pub use handlers::{app_router_with_vipps, solari_router_with_vipps};
#[cfg(all(feature = "vipps", feature = "stripe"))]
pub use handlers::{app_router_with_vipps_and_stripe, solari_router_with_vipps_and_stripe};
