use crate::{
    adapters::{
        stripe::models::{StripeConfig, StripeProvider},
        vipps::{adapter::VippsProvider, models::VippsConfig as InternalVippsConfig},
    },
    core::{PaymentProviderResponse, PaymentType},
    error::PaymentProviderError,
    traits::PaymentProvider,
};
use std::time::Duration;

pub use crate::adapters::vipps::models::VippsConfig;

pub struct SolariPaymentService {
    client: reqwest::Client,
    vipps_provider: Option<VippsProvider>,
    stripe_provider: Option<StripeProvider>,
}

#[derive(Debug, Clone)]
pub struct VippsTokenResponse {
    pub token: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone)]
pub struct VippsCreatePaymentResult {
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VippsPaymentStatusResult {
    pub reference: Option<String>,
    pub raw_status: String,
    pub status: crate::core::PaymentStatus,
}

#[derive(Debug, Clone)]
pub struct VippsPayRequest {
    pub amount: u32,
    pub return_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripePayRequest {
    pub amount: u32,
}

#[derive(Debug, Clone)]
pub enum PayRequest {
    Vipps(VippsPayRequest),
    Stripe(StripePayRequest),
}

impl SolariPaymentService {
    pub fn new() -> Result<Self, PaymentProviderError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(8)
            .tcp_keepalive(Duration::from_secs(60))
            .build()?;

        Ok(SolariPaymentService {
            client,
            vipps_provider: None,
            stripe_provider: None,
        })
    }

    pub fn vipps(&mut self, config: InternalVippsConfig) -> &mut Self {
        self.vipps_provider = Some(VippsProvider::new(self.client.clone(), config));
        self
    }

    /// Stripe is used for apple pay and visa payments.
    pub fn stripe(&mut self, config: StripeConfig) -> &mut Self {
        self.stripe_provider = Some(StripeProvider::new(self.client.clone(), config));
        self
    }

    pub async fn pay(
        &self,
        request: PayRequest,
    ) -> Result<PaymentProviderResponse, PaymentProviderError> {
        match request {
            PayRequest::Vipps(request) => {
                let provider = self
                    .vipps_provider
                    .as_ref()
                    .ok_or(PaymentProviderError::NotConfigured(PaymentType::Vipps))?;

                let response = provider
                    .create_payment(request.amount, request.return_url.as_deref())
                    .await?;

                Ok(PaymentProviderResponse {
                    provider: PaymentType::Vipps,
                    status: crate::core::PaymentStatus::Pending,
                    paid: 0,
                    reference: response.reference,
                    redirect_url: response.redirect_url,
                    return_url: request.return_url,
                })
            }
            PayRequest::Stripe(request) => {
                let provider = self
                    .stripe_provider
                    .as_ref()
                    .ok_or(PaymentProviderError::NotConfigured(PaymentType::Stripe))?;

                provider.pay(request.amount).await
            }
        }
    }

    // TODO - generic get payment status
    pub async fn vipps_get_payment_status(
        &self,
        reference: &str,
    ) -> Result<VippsPaymentStatusResult, PaymentProviderError> {
        let provider = self
            .vipps_provider
            .as_ref()
            .ok_or(PaymentProviderError::NotConfigured(PaymentType::Vipps))?;

        let (reference, raw_status, status) = provider.fetch_payment_status(reference).await?;

        Ok(VippsPaymentStatusResult {
            reference,
            raw_status,
            status,
        })
    }
}
