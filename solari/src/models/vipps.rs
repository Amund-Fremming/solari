#![cfg(feature = "vipps")]

use crate::models::PaymentStatus;

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
    pub status: PaymentStatus,
}

#[derive(Debug, Clone)]
pub struct VippsPayRequest {
    pub amount: u32,
    pub return_url: Option<String>,
}
