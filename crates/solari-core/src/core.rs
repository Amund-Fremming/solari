#[derive(Debug)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub struct PaymentProviderResponse {
    status: PaymentStatus,
    paid: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentProviderError {
    #[error("payment provider is not configured")]
    NotConfigured,

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
