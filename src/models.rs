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