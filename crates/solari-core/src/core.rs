use std::fmt;

#[derive(Debug)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub struct PaymentProviderResponse {
    pub status: PaymentStatus,
    pub paid: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentProviderError {
    #[error("payment provider {0} is not configured")]
    NotConfigured(PaymentType),

    #[error("invalid payment amount: {0}")]
    InvalidAmount(u32),

    #[error("payment provider authentication failed")]
    AuthenticationFailed,

    #[error("payment request timed out")]
    Timeout,

    #[error("payment provider is unavailable")]
    ProviderUnavailable,

    #[error("payment request failed: {0}")]
    RequestFailed(String),

    #[error("network error while contacting payment provider: {0}")]
    NetworkError(String),

    #[error("payment operation is not supported: {0}")]
    UnsupportedOperation(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
