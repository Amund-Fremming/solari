use axum::Router;
use solari_core::{PaymentProviderError, SolariPaymentService, VippsConfig};

use crate::{app_router, AppState};

pub struct SolariApi {
    payment_module: SolariPaymentService,
    domain: Option<String>,
    port: Option<u16>,
}

impl SolariApi {
    pub fn new() -> Result<Self, PaymentProviderError> {
        Ok(Self {
            payment_module: SolariPaymentService::new()?,
            domain: None,
            port: None,
        })
    }

    pub fn vipps(mut self, config: VippsConfig) -> Self {
        self.payment_module.vipps(config);
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn router(self) -> Router {
        app_router(AppState::new(self.payment_module))
    }

    pub async fn serve(self) -> Result<(), std::io::Error> {
        let SolariApi {
            payment_module,
            domain,
            port,
        } = self;

        let domain = domain.unwrap_or_else(|| "localhost".to_string());
        let port = port.unwrap_or(6767);

        let listener = tokio::net::TcpListener::bind(format!("{domain}:{port}")).await?;
        axum::serve(listener, app_router(AppState::new(payment_module))).await
    }
}
