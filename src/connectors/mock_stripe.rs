use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use crate::connectors::{ConnectorError, ConnectorResult };

pub async fn authorize(amount: i64, currency: &str, payment_method: &str) -> ConnectorResult<String> {
    info!("🔄 Mock Stripe: Processing payment of {} {} via {}", amount, currency, payment_method);
    
    let delay = Duration::from_millis(100 + (amount % 400) as u64);
    sleep(delay).await;

    let success = (amount % 10) < 8;

    if success {
        info!("✅ Mock Stripe: Payment authorized successfully");
        Ok("succeeded".to_string())
    } else {
        info!("❌ Mock Stripe: Payment authorization failed");
        Err(connectorError::AuthorizationFailed)
    }
}

pub async fn capture(payment_id: &str) -> ConnectorResult<String> {
    info!("🔄 Mock Stripe: Capturing payment {}", payment_id);

    sleep(Duration::from_millis(50 + (payment_id.len() as u64 * 10))).await;

    let success = (payment_id.len() % 20 ) < 19;

    if success {
        info!("✅ Mock Stripe: Payment captured successfully");
        Ok("succeeded".to_string())
    } else {
        info!(" Mock Stripe: Payment capture failed");
        Err(connectorError::NetworkError)
    }
}