pub mod apple_pay;
pub mod stripe;
pub mod vipps;

use axum::Router;
use solari_core::{PaymentProviderError, SolariPaymentService, VippsConfig};

use crate::models::AppState;

pub fn solari_router(state: AppState) -> Router {
	Router::new()
		.nest("/vipps", vipps::router())
		.nest("/apple-pay", apple_pay::router())
		.nest("/stripe", stripe::router())
		.with_state(state)
}

pub fn app_router(state: AppState) -> Router {
	Router::new().nest("/solari", solari_router(state))
}

pub fn solari_router_with_vipps(vipps_config: VippsConfig) -> Result<Router, PaymentProviderError> {
	let mut payment_module = SolariPaymentService::new()?;
	payment_module.vipps(vipps_config);

	Ok(solari_router(AppState::new(payment_module)))
}

pub fn app_router_with_vipps(vipps_config: VippsConfig) -> Result<Router, PaymentProviderError> {
	let mut payment_module = SolariPaymentService::new()?;
	payment_module.vipps(vipps_config);

	Ok(app_router(AppState::new(payment_module)))
}
