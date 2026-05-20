use std::future::Future;
use std::sync::Arc;

use axum::Router;

use crate::{
    handlers::app_router,
    models::AppState,
    webhooks::{SolariHandlers, WebhookEvent},
    PaymentProviderError, PaymentProviderResponse, SolariPaymentService,
};

#[cfg(feature = "vipps")]
use crate::{
    webhooks::VippsWebhookPayload,
    VippsConfig,
};

#[cfg(feature = "stripe")]
use crate::{
    webhooks::StripeWebhookPayload,
    StripeConfig,
};

pub struct SolariRouter {
    payment_module: SolariPaymentService,
    handlers: SolariHandlers,
}

impl SolariRouter {
    pub fn new() -> Result<Self, PaymentProviderError> {
        Ok(Self {
            payment_module: SolariPaymentService::new()?,
            handlers: SolariHandlers::default(),
        })
    }

    #[cfg(feature = "vipps")]
    pub fn vipps(mut self, config: VippsConfig) -> Self {
        self.payment_module.vipps(config);
        self
    }

    #[cfg(feature = "stripe")]
    pub fn stripe(mut self, config: StripeConfig) -> Self {
        self.payment_module.stripe(config);
        self
    }

    pub fn on_pay<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(PaymentProviderResponse) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.handlers.on_pay = Some(Arc::new(move |p| Box::pin(f(p))));
        self
    }

    #[cfg(feature = "vipps")]
    pub fn on_vipps_webhook<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(WebhookEvent<VippsWebhookPayload>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.handlers.on_vipps_webhook = Some(Arc::new(move |e| Box::pin(f(e))));
        self
    }

    #[cfg(feature = "stripe")]
    pub fn on_stripe_webhook<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(WebhookEvent<StripeWebhookPayload>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.handlers.on_stripe_webhook = Some(Arc::new(move |e| Box::pin(f(e))));
        self
    }

    pub fn build(self) -> Router {
        let state = AppState::with_handlers(self.payment_module, self.handlers);
        app_router(state)
    }
}

pub type SolariApi = SolariRouter;

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

    #[cfg(feature = "vipps")]
    pub fn vipps(mut self, config: VippsConfig) -> Self {
        self.payment_module.vipps(config);
        self
    }

    #[cfg(feature = "stripe")]
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

    pub fn router(self) -> Router {
        let payment_module = self.build_payment_service();
        app_router(AppState::new(payment_module))
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
