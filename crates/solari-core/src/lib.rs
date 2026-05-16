mod adapters;
mod core;
mod error;
pub mod payment_module;
mod traits;

pub use core::{PaymentProviderResponse, PaymentStatus, PaymentType};
pub use error::PaymentProviderError;
pub use payment_module::{
    PayRequest, SolariPaymentService, VippsConfig, VippsCreatePaymentResult,
    VippsPaymentStatusResult, VippsTokenResponse,
};
