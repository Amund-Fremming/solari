use std::time::SystemTimeError;

use crate::models::PaymentType;

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

#[cfg(any(feature = "vipps", feature = "stripe"))]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
#[cfg(any(feature = "vipps", feature = "stripe"))]
use serde::Serialize;

#[cfg(any(feature = "vipps", feature = "stripe"))]
pub type ApiResult<T> = Result<T, SolariApiError>;

#[cfg(any(feature = "vipps", feature = "stripe"))]
#[derive(Debug, thiserror::Error)]
pub enum SolariApiError {
    #[error("{0}")]
    Payment(#[from] PaymentProviderError),

    #[error("{0}")]
    BadRequest(String),
}

#[cfg(any(feature = "vipps", feature = "stripe"))]
#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

#[cfg(any(feature = "vipps", feature = "stripe"))]
impl IntoResponse for SolariApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            SolariApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            SolariApiError::Payment(err) => match err {
                PaymentProviderError::InvalidAmount(_)
                | PaymentProviderError::NotConfigured(_)
                | PaymentProviderError::UnsupportedOperation(_) => StatusCode::BAD_REQUEST,
                PaymentProviderError::AuthenticationFailed => StatusCode::UNAUTHORIZED,
                PaymentProviderError::Timeout => StatusCode::GATEWAY_TIMEOUT,
                PaymentProviderError::ProviderUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                PaymentProviderError::RequestFailed(_)
                | PaymentProviderError::NetworkError(_)
                | PaymentProviderError::Request(_) => StatusCode::BAD_GATEWAY,
                PaymentProviderError::TimeError(_)
                | PaymentProviderError::ReadLockError(_)
                | PaymentProviderError::WriteLockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };

        let body = ErrorBody {
            error: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}
