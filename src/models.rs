use serde::{Deserialize, Serialize};
use chrono::{DateTime , Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub amount: i64,
    pub currency: String,
    pub payment_method: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub payment_id: String,
    pub status: PaymentStatus,
    pub amount: i64,
    pub currency: String,
    pub connector_used: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PaymentStatus {
    Processing,
    Succeeded,
    Failed,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Processing => write!(f, "processing"),
            PaymentStatus::Succeeded => write!(f, "succeeded"),
            PaymentStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub payment_method: String,
    pub status: PaymentStatus,
    pub connector_used: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Payment {
    pub fn new(
        amount: i64,
        currency: String,
        payment_method: String,
        connector_used: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            amount,
            currency,
            payment_method,
            status: PaymentsStatus::Processing,
            connector_used,
            created_at: now,
            updated_at: now,
        }
    }
}