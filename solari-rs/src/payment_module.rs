use crate::{
    adapters::{
        stripe::{adapter::StripeAdapter, models::StripePaymentRequest},
        vipps::adapter::VippsAdapter,
    },
    models::{PaymentProvider, PaymentResponse},
    PaymentStatus, SolariError,
};
#[cfg(any(feature = "vipps", feature = "stripe"))]
use std::time::Duration;
use tracing::{error, info};

#[cfg(feature = "stripe")]
use crate::adapters::stripe::models::StripeConfig as InternalStripeConfig;

#[cfg(feature = "vipps")]
use crate::adapters::vipps::models::VippsConfig as InternalVippsConfig;

pub use crate::adapters::stripe::models::StripeConfig;
pub use crate::adapters::vipps::models::VippsConfig;

pub struct SolariPaymentService {
    #[cfg(any(feature = "vipps", feature = "stripe"))]
    client: reqwest::Client,
    vipps_provider: Option<VippsAdapter>,
    stripe_provider: Option<StripeAdapter>,
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
pub struct VippsPaymentResult {
    pub reference: Option<String>,
    pub raw_status: String,
    pub status: PaymentStatus,
    pub currency: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VippsPayRequest {
    pub amount: u32,
    pub return_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripePayRequest {
    pub amount: u32,
    pub currency: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripePaymentResponse {
    pub provider: PaymentProvider,
    pub flow: StripePaymentFlowType,
    pub status: String,
    pub amount: u32,
    pub currency: String,
    pub payment_intent_id: String,
}

#[derive(Debug, Clone, Copy)]
pub enum StripePaymentFlowType {
    Card,
    ApplePay,
}

#[derive(Debug, Clone)]
pub enum PayRequest {
    Vipps(VippsPayRequest),
    Stripe(StripePayRequest),
}

impl SolariPaymentService {
    pub fn new() -> Result<Self, SolariError> {
        #[cfg(any(feature = "vipps", feature = "stripe"))]
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(8)
            .tcp_keepalive(Duration::from_secs(60))
            .build()?;

        Ok(SolariPaymentService {
            #[cfg(any(feature = "vipps", feature = "stripe"))]
            client,
            vipps_provider: None,
            stripe_provider: None,
        })
    }

    #[cfg(feature = "vipps")]
    pub fn vipps(&mut self, config: InternalVippsConfig) -> &mut Self {
        self.vipps_provider = Some(VippsAdapter::new(self.client.clone(), config));
        self
    }

    #[cfg(feature = "stripe")]
    pub fn stripe(&mut self, config: InternalStripeConfig) -> &mut Self {
        self.stripe_provider = Some(StripeAdapter::new(self.client.clone(), config));
        self
    }

    pub async fn pay(&self, request: PayRequest) -> Result<PaymentResponse, SolariError> {
        match request {
            PayRequest::Vipps(request) => {
                let provider = self
                    .vipps_provider
                    .as_ref()
                    .ok_or(SolariError::NotConfigured(PaymentProvider::Vipps))?;

                let response = provider
                    .create_payment(request.amount, request.return_url.as_deref())
                    .await?;

                Ok(PaymentResponse {
                    provider: PaymentProvider::Vipps,
                    status: crate::models::PaymentStatus::Pending,
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
                    .ok_or(SolariError::NotConfigured(PaymentProvider::Stripe))?;

                provider
                    .create_payment(StripePaymentRequest {
                        amount: request.amount,
                        currency: request.currency.unwrap_or_else(|| "nok".to_string()),
                        description: request.description,
                    })
                    .await
                    .map(|intent| PaymentResponse {
                        provider: PaymentProvider::Stripe,
                        status: crate::models::PaymentStatus::Pending,
                        paid: 0,
                        reference: Some(intent.id),
                        redirect_url: None,
                        return_url: None,
                    })
            }
        }
    }

    pub async fn vipps_fetch_access_token(&self) -> Result<VippsTokenResponse, SolariError> {
        let provider = self
            .vipps_provider
            .as_ref()
            .ok_or(SolariError::NotConfigured(PaymentProvider::Vipps))?;

        let token = provider.fetch_access_token().await?;

        Ok(VippsTokenResponse {
            token: token.token,
            expires_at: token.expires_at,
        })
    }

    pub async fn vipps_get_valid_token(&self) -> Result<VippsTokenResponse, SolariError> {
        let provider = self
            .vipps_provider
            .as_ref()
            .ok_or(SolariError::NotConfigured(PaymentProvider::Vipps))?;

        let token = provider.get_valid_token().await?;

        Ok(VippsTokenResponse {
            token: token.token,
            expires_at: token.expires_at,
        })
    }

    pub async fn stripe_create_card_payment_intent(
        &self,
        request: StripePayRequest,
    ) -> Result<StripePaymentResponse, SolariError> {
        self.stripe_create_payment_intent(request, StripePaymentFlowType::Card)
            .await
    }

    pub async fn stripe_create_apple_pay_payment_intent(
        &self,
        request: StripePayRequest,
    ) -> Result<StripePaymentResponse, SolariError> {
        self.stripe_create_payment_intent(request, StripePaymentFlowType::ApplePay)
            .await
    }

    async fn stripe_create_payment_intent(
        &self,
        request: StripePayRequest,
        flow: StripePaymentFlowType,
    ) -> Result<StripePaymentResponse, SolariError> {
        let flow_label = match flow {
            StripePaymentFlowType::Card => "card",
            StripePaymentFlowType::ApplePay => "apple_pay",
        };

        let provider = self
            .stripe_provider
            .as_ref()
            .ok_or(SolariError::NotConfigured(PaymentProvider::Stripe))?;

        let currency = request
            .currency
            .clone()
            .unwrap_or_else(|| "nok".to_string());

        info!(
            "Solari service creating Stripe intent: flow={} amount={} currency={}",
            flow_label,
            request.amount,
            request.currency.as_deref().unwrap_or("nok")
        );

        let intent_result = provider
            .create_payment(StripePaymentRequest {
                amount: request.amount,
                currency: currency.clone(),
                description: request.description,
            })
            .await;

        let intent = match intent_result {
            Ok(intent) => intent,
            Err(error_value) => {
                error!(
                    "Solari service Stripe intent failed: flow={} amount={} error={}",
                    flow_label, request.amount, error_value
                );
                return Err(error_value);
            }
        };

        info!(
            "Solari service Stripe intent ready: flow={} intent_id={} status={}",
            flow_label, intent.id, intent.status
        );

        Ok(StripePaymentResponse {
            provider: PaymentProvider::Stripe,
            flow,
            status: intent.status,
            amount: intent.amount,
            currency: intent.currency,
            payment_intent_id: intent.id,
        })
    }

    pub async fn vipps_get_payment_status(
        &self,
        reference: &str,
    ) -> Result<VippsPaymentResult, SolariError> {
        let provider = self
            .vipps_provider
            .as_ref()
            .ok_or(SolariError::NotConfigured(PaymentProvider::Vipps))?;

        let (reference, raw_status, status, currency) =
            provider.fetch_payment_status(reference).await?;

        Ok(VippsPaymentResult {
            reference,
            raw_status,
            status,
            currency,
        })
    }
}
