pub mod storage;

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "api")]
pub mod error;
#[cfg(feature = "api")]
pub mod handlers;
#[cfg(feature = "api")]
pub mod models;

#[cfg(feature = "api")]
pub use api::SolariApi;
#[cfg(feature = "api")]
pub use error::{ApiResult, SolariApiError};
#[cfg(feature = "api")]
pub use handlers::{app_router, app_router_with_vipps, solari_router, solari_router_with_vipps};
#[cfg(feature = "api")]
pub use models::AppState;
