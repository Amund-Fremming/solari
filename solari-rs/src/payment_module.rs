use crate::{
    adapters::{
        stripe::models::{
            StripeCreatePaymentIntentRequest, StripePaymentFlow, StripePaymentIntentResult,
            StripeProvider,
        },
        vipps::adapter::VippsProvider,
    },
    error::PaymentProviderError,
    models::{PaymentProviderResponse, PaymentType},
};
use std::time::Duration;
use tracing::{error, info};

#[cfg(feature = "stripe")]
use crate::adapters::stripe::models::StripeConfig as InternalStripeConfig;

#[cfg(feature = "vipps")]
use crate::adapters::vipps::models::VippsConfig as InternalVippsConfig;

pub use crate::adapters::stripe::models::StripeConfig;
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
    pub status: crate::models::PaymentStatus,
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
pub struct StripePaymentIntentResponse {
    pub provider: PaymentType,
    pub flow: StripePaymentFlowType,
    pub status: String,
    pub amount: u32,
    pub currency: String,
    pub payment_intent_id: String,
    pub client_secret: String,
    pub publishable_key: String,
    pub account_id: Option<String>,
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

    #[cfg(feature = "vipps")]
    pub fn vipps(&mut self, config: InternalVippsConfig) -> &mut Self {
        self.vipps_provider = Some(VippsProvider::new(self.client.clone(), config));
        self
    }

    #[cfg(feature = "stripe")]
    pub fn stripe(&mut self, config: InternalStripeConfig) -> &mut Self {
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
                    .ok_or(PaymentProviderError::NotConfigured(PaymentType::Stripe))?;

                provider
                    .create_payment_intent(StripeCreatePaymentIntentRequest {
                        amount: request.amount,
                        currency: request.currency.unwrap_or_else(|| "nok".to_string()),
                        description: request.description,
                        flow: StripePaymentFlow::Card,
                    })
                    .await
                    .map(|intent| PaymentProviderResponse {
                        provider: PaymentType::Stripe,
                        status: crate::models::PaymentStatus::Pending,
                        paid: 0,
                        reference: Some(intent.id),
                        redirect_url: None,
                        return_url: None,
                    })
            }
        }
    }

    pub async fn vipps_fetch_access_token(
        &self,
    ) -> Result<VippsTokenResponse, PaymentProviderError> {
        let provider = self
            .vipps_provider
            .as_ref()
            .ok_or(PaymentProviderError::NotConfigured(PaymentType::Vipps))?;

        let token = provider.fetch_access_token().await?;

        Ok(VippsTokenResponse {
            token: token.token,
            expires_at: token.expires_at,
        })
    }

    pub async fn vipps_get_valid_token(&self) -> Result<VippsTokenResponse, PaymentProviderError> {
        let provider = self
            .vipps_provider
            .as_ref()
            .ok_or(PaymentProviderError::NotConfigured(PaymentType::Vipps))?;

        let token = provider.get_valid_token().await?;

        Ok(VippsTokenResponse {
            token: token.token,
            expires_at: token.expires_at,
        })
    }

    pub async fn stripe_create_card_payment_intent(
        &self,
        request: StripePayRequest,
    ) -> Result<StripePaymentIntentResponse, PaymentProviderError> {
        self.stripe_create_payment_intent(request, StripePaymentFlow::Card)
            .await
    }

    pub async fn stripe_create_apple_pay_payment_intent(
        &self,
        request: StripePayRequest,
    ) -> Result<StripePaymentIntentResponse, PaymentProviderError> {
        self.stripe_create_payment_intent(request, StripePaymentFlow::ApplePay)
            .await
    }

    async fn stripe_create_payment_intent(
        &self,
        request: StripePayRequest,
        flow: StripePaymentFlow,
    ) -> Result<StripePaymentIntentResponse, PaymentProviderError> {
        let flow_label = match flow {
            StripePaymentFlow::Card => "card",
            StripePaymentFlow::ApplePay => "apple_pay",
        };

        let provider = self
            .stripe_provider
            .as_ref()
            .ok_or(PaymentProviderError::NotConfigured(PaymentType::Stripe))?;

        info!(
            "Solari service creating Stripe intent: flow={} amount={} currency={}",
            flow_label,
            request.amount,
            request.currency.as_deref().unwrap_or("nok")
        );

        let intent_result = provider
            .create_payment_intent(StripeCreatePaymentIntentRequest {
                amount: request.amount,
                currency: request.currency.unwrap_or_else(|| "nok".to_string()),
                description: request.description,
                flow,
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

        Ok(map_intent_response(intent, flow))
    }

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

fn map_intent_response(
    intent: StripePaymentIntentResult,
    flow: StripePaymentFlow,
) -> StripePaymentIntentResponse {
    StripePaymentIntentResponse {
        provider: PaymentType::Stripe,
        flow: match flow {
            StripePaymentFlow::Card => StripePaymentFlowType::Card,
            StripePaymentFlow::ApplePay => StripePaymentFlowType::ApplePay,
        },
        status: intent.status,
        amount: intent.amount,
        currency: intent.currency,
        payment_intent_id: intent.id,
        client_secret: intent.client_secret,
        publishable_key: intent.publishable_key,
        account_id: intent.account_id,
    }
}
