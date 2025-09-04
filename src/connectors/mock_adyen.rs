use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use crate::connectors::{ConnectorError, ConnectorResult};

pub async fn authorize(amount: i64, currency: &str, payment_method: &str) -> ConnectorResult<String> {
    info!("🔄 Mock Adyen: Processing payment of {} {} via {}", amount, currency, payment_method);

    let delay = Duration::from_millis(150 + (amount % 450) as u64);
    sleep(delay).await;

    let success = (amount % 12) < 9;

    if success {
        info!("✅ Mock Adyen: Payment authorized successfully");
        Ok("succeeded".to_string())
    } else{
        info!("❌ Mock Adyen: Payment authorization failed");
        Err(ConnectorError::AuthorizationFailed)
    }
}

pub async fn capture(payment_id: &str) -> ConnectorResult<String> {
    info!("🔄 Mock Adyen: Capturing payment {}", payment_id);

    sleep(Duration::from_millis(75 + (payment_id.len() as u64 * 15))).await;

    let success = (payment_id.len() % 10) < 9;

    if success {
        info!("✅ Mock Adyen: Payment captured successfully");
        Ok("succeeded".to_string())
    } else {
        info!("❌ Mock Adyen: Payment capture failed");
        Err(ConnectorError::NetworkError)
    }
}