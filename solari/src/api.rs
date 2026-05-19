use axum::Router;

use crate::{
    app_router, AppState, PaymentProviderError, SolariPaymentService, StripeConfig, VippsConfig,
};

pub struct Solari {
    payment_module: SolariPaymentService,
    domain: Option<String>,
    port: Option<u16>,
}

impl Solari {
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

    pub fn stripe(mut self, config: StripeConfig) -> Self {
        self.payment_module.stripe(config);
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

    pub fn build_payment_service(self) -> SolariPaymentService {
        self.payment_module
    }

    pub fn build_router(self) -> Router {
        let payment_module = self.build_payment_service();
        app_router(AppState::new(payment_module))
    }

    pub fn router(self) -> Router {
        self.build_router()
    }

    pub async fn serve(self) -> Result<(), std::io::Error> {
        let Solari {
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

pub type SolariApi = Solari;
