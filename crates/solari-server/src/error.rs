use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use solari_core::PaymentProviderError;

pub type ApiResult<T> = Result<T, SolariApiError>;

#[derive(Debug, thiserror::Error)]
pub enum SolariApiError {
    #[error("{0}")]
    Payment(#[from] PaymentProviderError),

    #[error("{0}")]
    BadRequest(String),
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

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
