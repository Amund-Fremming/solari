use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct CachedToken {
    pub token: String,
    pub expires_at: u64,
}

#[derive(Debug)]
pub struct VippsConfig {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub subscription_key: String,
    pub merchant_serial_number: String,
}

impl VippsConfig {
    pub fn new(
        base_url: String,
        client_id: String,
        client_secret: String,
        subscription_key: String,
        merchant_serial_number: String,
    ) -> Self {
        Self {
            base_url,
            client_id,
            client_secret,
            subscription_key,
            merchant_serial_number,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VippsAccessTokenResponse {
    #[serde(alias = "accessToken")]
    pub access_token: String,

    #[serde(default)]
    expires_in: Option<VippsExpiresIn>,
}

impl VippsAccessTokenResponse {
    pub fn expires_in_seconds(&self) -> u64 {
        match &self.expires_in {
            Some(VippsExpiresIn::Number(value)) => *value,
            Some(VippsExpiresIn::Text(value)) => value.parse::<u64>().unwrap_or(3600),
            None => 3600,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum VippsExpiresIn {
    Number(u64),
    Text(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsCreatePaymentRequest {
    #[serde(skip_serializing)]
    pub request_id: String,
    pub amount: VippsAmount,
    pub payment_method: VippsPaymentMethod,
    pub reference: String,
    pub return_url: String,
    pub user_flow: String,
}

impl VippsCreatePaymentRequest {
    pub fn new(amount: u32, return_url: String, user_flow: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Convert NOK to øre (minor units): 1 NOK = 100 øre
        let amount_in_ore = amount * 100;

        Self {
            request_id: format!("solari-req-{now}-{amount}"),
            amount: VippsAmount {
                currency: "NOK".to_string(),
                value: amount_in_ore,
            },
            payment_method: VippsPaymentMethod {
                kind: "WALLET".to_string(),
            },
            reference: format!("solari-{now}-{amount}"),
            return_url,
            user_flow,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsCreatePaymentResponse {
    pub reference: Option<String>,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VippsAmount {
    pub currency: String,
    pub value: u32,
}

#[derive(Debug, Serialize)]
pub struct VippsPaymentMethod {
    #[serde(rename = "type")]
    pub kind: String,
}
