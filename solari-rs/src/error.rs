use std::time::SystemTimeError;

use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::models::PaymentProvider;

#[derive(Debug, thiserror::Error)]
pub enum SolariError {
    #[error("payment provider {0} is not configured")]
    NotConfigured(PaymentProvider),

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

    #[error("api error: {1}")]
    ApiError(StatusCode, String),
}

impl IntoResponse for SolariError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            SolariError::NotConfigured(payment_provider) => (
                StatusCode::BAD_REQUEST,
                format!("payment provider {payment_provider} is not configured"),
            ),
            SolariError::InvalidAmount(amount) => (
                StatusCode::BAD_REQUEST,
                format!("invalid payment amount: {amount}"),
            ),
            SolariError::AuthenticationFailed => (
                StatusCode::UNAUTHORIZED,
                "payment provider authentication failed".to_string(),
            ),
            SolariError::Timeout => (
                StatusCode::GATEWAY_TIMEOUT,
                "payment request timed out".to_string(),
            ),
            SolariError::ProviderUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "payment provider is unavailable".to_string(),
            ),
            SolariError::RequestFailed(reason) => (
                StatusCode::BAD_GATEWAY,
                format!("payment request failed: {reason}"),
            ),
            SolariError::NetworkError(reason) => (
                StatusCode::BAD_GATEWAY,
                format!("network error while contacting payment provider: {reason}"),
            ),
            SolariError::Request(error) => (
                StatusCode::BAD_GATEWAY,
                format!("http request failed: {error}"),
            ),
            SolariError::UnsupportedOperation(operation) => (
                StatusCode::NOT_IMPLEMENTED,
                format!("payment operation is not supported: {operation}"),
            ),
            SolariError::TimeError(system_time_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("time calculation error: {system_time_error}"),
            ),
            SolariError::ReadLockError(reason) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to acquire read lock: {reason}"),
            ),
            SolariError::WriteLockError(reason) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to acquire write lock: {reason}"),
            ),
            SolariError::ApiError(status, message) => (status, message),
        };

        (status, message).into_response()
    }
}
