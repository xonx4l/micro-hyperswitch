use anyhow::Result;
use tracing::info;
use std::collection::HashMap;
use std::sync::Mutex;
use crate::models::{Payment, PaymentStatus};

pub struct Database {
    payments: Mutex<HashMap<String, Payments>>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        info!("Initializing in-memory database...");
        Ok(Self {
            payments: Mutex::new(HashMap::new()),
        })
    }

    pub async fn create_payment(&self, payment: &Payment) -> Result<()> {
        let mut payments = self.payments.lock().unwrap();
        payments.insert(payment.id.to_string(), payment.clone());
        info!("Payment stored in memory: {}", payment.id);
        Ok(())
    }

    pub async fn update_payment_status(&self, payment_id: &str, status: PaymentStatus) -> Result<()> {
        let mut payments = self.payments.lock().unwrap();
        if let Some(payment) = payments.get_mut(payment_id) {
            payment.status = status;
            payment.updated_at = chrono::Utc::now();
            info!("Payment status updated: {} -> {:?}", payment_id, payment.status);
        }
        Ok(())        
    }

    pub async fn get_payment(&self, payment_id: &str) -> Result<Option<Payment>> {
         let payments = self.payments.lock().unwrap();
         Ok(payments.get(payment_id).cloned())
    }
}