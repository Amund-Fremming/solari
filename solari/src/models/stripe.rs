#![cfg(feature = "stripe")]

use crate::models::PaymentType;

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
