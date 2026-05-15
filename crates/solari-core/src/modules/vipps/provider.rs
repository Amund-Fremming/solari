use std::{
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use tracing::{error, info};

use crate::{
    core::{PaymentProviderResponse, PaymentStatus},
    error::PaymentProviderError,
    modules::vipps::models::{
        CachedToken, VippsAccessTokenResponse, VippsConfig, VippsCreatePaymentRequest,
        VippsCreatePaymentResponse,
    },
    traits::PaymentProvider,
};

#[derive(Debug)]
pub struct VippsProvider {
    client: reqwest::Client,
    config: VippsConfig,
    token_cache: Arc<RwLock<Option<CachedToken>>>,
}

impl VippsProvider {
    pub fn new(client: reqwest::Client, config: VippsConfig) -> Self {
        Self {
            client,
            config,
            token_cache: Arc::new(RwLock::new(None)),
        }
    }

    fn now() -> Result<u64, PaymentProviderError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())?;
        Ok(now)
    }

    pub async fn get_valid_token(&self) -> Result<CachedToken, PaymentProviderError> {
        {
            let lock = self
                .token_cache
                .read()
                .map_err(|e| PaymentProviderError::ReadLockError(e.to_string()))?;

            if let Some(token) = &*lock {
                let now = Self::now()?;
                if token.expires_at > now {
                    return Ok(token.clone());
                }
            }
        }

        let token = self.fetch_access_token().await?;

        let mut lock = self
            .token_cache
            .write()
            .map_err(|e| PaymentProviderError::WriteLockError(e.to_string()))?;

        *lock = Some(token.clone());
        Ok(token)
    }

    pub async fn fetch_access_token(&self) -> Result<CachedToken, PaymentProviderError> {
        let url = format!(
            "{}/accesstoken/get",
            self.config.base_url.trim_end_matches('/')
        );

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("client_id", &self.config.client_id)
            .header("client_secret", &self.config.client_secret)
            .header("Ocp-Apim-Subscription-Key", &self.config.subscription_key)
            .header(
                "Merchant-Serial-Number",
                &self.config.merchant_serial_number,
            )
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or("No body".to_string());
            return Err(PaymentProviderError::RequestFailed(format!(
                "vipps access token request failed ({status}): {body}"
            )));
        }

        let payload: VippsAccessTokenResponse = response.json().await?;
        if payload.access_token.trim().is_empty() {
            error!("Received a empty access token");
            return Err(PaymentProviderError::AuthenticationFailed);
        }

        let now = Self::now()?;

        let cached_token = CachedToken {
            token: payload.access_token.clone(),
            expires_at: now + payload.expires_in_seconds(),
        };

        Ok(cached_token)
    }

    pub async fn create_payment(
        &self,
        amount: u32,
        return_url: Option<&str>,
    ) -> Result<VippsCreatePaymentResponse, PaymentProviderError> {
        if amount == 0 {
            return Err(PaymentProviderError::InvalidAmount(amount));
        }

        let token = self.get_valid_token().await?;
        let url = format!(
            "{}/epayment/v1/payments",
            self.config.base_url.trim_end_matches('/')
        );

        let payload = VippsCreatePaymentRequest::new(
            amount,
            return_url
                .unwrap_or("https://example.com/checkout/complete")
                .to_string(),
            "WEB_REDIRECT".to_string(),
        );

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header(
                "Merchant-Serial-Number",
                &self.config.merchant_serial_number,
            )
            .header("Ocp-Apim-Subscription-Key", &self.config.subscription_key)
            .header("Idempotency-Key", &payload.request_id)
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or("No body".to_string());
            return Err(PaymentProviderError::RequestFailed(format!(
                "vipps create payment failed ({status}): {body}"
            )));
        }

        let payload: VippsCreatePaymentResponse = response.json().await?;

        info!("💵 Vipps payment created: {amount} kr");

        Ok(payload)
    }
}

#[async_trait]
impl PaymentProvider for VippsProvider {
    async fn pay(&self, amount: u32) -> Result<PaymentProviderResponse, PaymentProviderError> {
        self.create_payment(amount, None).await?;

        Ok(PaymentProviderResponse {
            status: PaymentStatus::Pending,
            paid: amount,
        })
    }
}
