mod adapters;
mod error;
mod models;
mod payment_module;
mod traits;

pub mod storage;

pub use error::PaymentProviderError;
pub use models::{PaymentProviderResponse, PaymentStatus, PaymentType};
pub use payment_module::{
    PayRequest, SolariPaymentService, StripeConfig, StripePayRequest, StripePaymentFlowType,
    StripePaymentIntentResponse, VippsConfig, VippsCreatePaymentResult, VippsPayRequest,
    VippsPaymentStatusResult, VippsTokenResponse,
};

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "api")]
pub mod handlers;

#[cfg(feature = "api")]
pub use api::{Solari, SolariApi};
#[cfg(feature = "api")]
pub use error::{ApiResult, SolariApiError};
#[cfg(feature = "api")]
pub use handlers::{app_router, app_router_with_vipps, solari_router, solari_router_with_vipps};
#[cfg(feature = "api")]
pub use handlers::{app_router_with_vipps_and_stripe, solari_router_with_vipps_and_stripe};
#[cfg(feature = "api")]
pub use models::AppState;
