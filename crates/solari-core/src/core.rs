use std::fmt;

#[derive(Debug)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub struct PaymentProviderResponse {
    pub status: PaymentStatus,
    pub paid: u32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PaymentType {
    Vipps,
    ApplePay,
    Stripe,
}

impl fmt::Display for PaymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PaymentType::Vipps => "vipps",
            PaymentType::ApplePay => "apple_pay",
            PaymentType::Stripe => "stripe",
        };

        f.write_str(name)
    }
}
