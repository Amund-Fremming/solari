use std::time::SystemTimeError;

use crate::core::PaymentType;

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

    #[error("http request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("payment operation is not supported: {0}")]
    UnsupportedOperation(&'static str),

    #[error("time calculation error: {0}")]
    TimeError(#[from] SystemTimeError),

    #[error("failed to acquire read lock: {0}")]
    ReadLockError(String),

    #[error("failed to acquire write lock: {0}")]
    WriteLockError(String),
}
