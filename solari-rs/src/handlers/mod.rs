pub mod solari;

use axum::Router;

use crate::models::AppState;

#[cfg(feature = "vipps")]
use crate::{SolariError, SolariPaymentService, VippsConfig};

#[cfg(all(feature = "vipps", feature = "stripe"))]
use crate::StripeConfig;

pub fn solari_router(state: AppState) -> Router {
    solari::router().with_state(state)
}

pub fn app_router(state: AppState) -> Router {
    Router::new().nest("/solari", solari_router(state))
}

#[cfg(feature = "vipps")]
pub fn solari_router_with_vipps(vipps_config: VippsConfig) -> Result<Router, SolariError> {
    let mut payment_module = SolariPaymentService::new()?;
    payment_module.vipps(vipps_config);

    Ok(solari_router(AppState::new(payment_module)))
}

#[cfg(feature = "vipps")]
pub fn app_router_with_vipps(vipps_config: VippsConfig) -> Result<Router, SolariError> {
    let mut payment_module = SolariPaymentService::new()?;
    payment_module.vipps(vipps_config);

    Ok(app_router(AppState::new(payment_module)))
}

#[cfg(all(feature = "vipps", feature = "stripe"))]
pub fn solari_router_with_vipps_and_stripe(
    vipps_config: VippsConfig,
    stripe_config: StripeConfig,
) -> Result<Router, SolariError> {
    let mut payment_module = SolariPaymentService::new()?;
    payment_module.vipps(vipps_config);
    payment_module.stripe(stripe_config);

    Ok(solari_router(AppState::new(payment_module)))
}

#[cfg(all(feature = "vipps", feature = "stripe"))]
pub fn app_router_with_vipps_and_stripe(
    vipps_config: VippsConfig,
    stripe_config: StripeConfig,
) -> Result<Router, SolariError> {
    let mut payment_module = SolariPaymentService::new()?;
    payment_module.vipps(vipps_config);
    payment_module.stripe(stripe_config);

    Ok(app_router(AppState::new(payment_module)))
}
