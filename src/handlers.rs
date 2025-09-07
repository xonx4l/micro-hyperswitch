use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::info;

use crate::{
    models::{PaymentRequest, PaymentResponse, Payment, PaymentStatus},
    database::Database,
    connectors::{self, ConnectorError},
};

pub async fn payment_handler(
    State(db): State<Arc<Database>>,
    Json(payload): Json<PaymentRequest>,
) -> Result<Json<PaymentResponse>, StatusCode> {
    info!("💳 Processing payment request: {} {} via {}", 
          payload.amount, payload.currency, payload.payment_method);
    
    let connector_name = connectors::select_connector(&payload.currency).await;
    
    let payment = Payment::new(
        payload.amount,
        payload.currency.clone(),
        payload.payment_method.clone(),
        connector_name.to_string(),
    );
    
    if let Err(e) = db.create_payment(&payment).await {
        info!("❌ Failed to create payment in database: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let result = match connector_name {
        "stripe" => {
            crate::connectors::mock_stripe::authorize(
                payload.amount,
                &payload.currency,
                &payload.payment_method,
            ).await
        }
        "adyen" => {
            crate::connectors::mock_adyen::authorize(
                payload.amount,
                &payload.currency,
                &payload.payment_method,
            ).await
        }
        _ => {
            info!("❌ Unknown connector: {}", connector_name);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    let status = match result {
        Ok(_) => {
            info!("✅ Payment processed successfully");
            PaymentStatus::Succeeded
        }
        Err(ConnectorError::AuthorizationFailed) => {
            info!("❌ Payment authorization failed");
            PaymentStatus::Failed
        }
        Err(ConnectorError::NetworkError) => {
            info!("❌ Network error during payment processing");
            PaymentStatus::Failed
        }
        Err(ConnectorError::InvalidAmount) => {
            info!("❌ Invalid amount provided");
            PaymentStatus::Failed
        }
    };
    
    if let Err(e) = db.update_payment_status(&payment.id.to_string(), status.clone()).await {
        info!("❌ Failed to update payment status: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let response = PaymentResponse {
        payment_id: payment.id.to_string(),
        status,
        amount: payment.amount,
        currency: payment.currency,
        connector_used: payment.connector_used,
        created_at: payment.created_at,
    };
    
    info!("✅ Payment response prepared: {:?}", response);
    Ok(Json(response))
} 
 