use tracing::info;

pub mod mock_stripe;
pub mod mock_adyen;

#[derive(Debug)]
pub enum ConnectorError {
    AuthorizationFailed,
    NetworkError,
    InvalidAmount,
}

impl std::fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectorError::AuthorizationFailed => write!(f, "Authorization failed"),
            ConnectorError::NetworkError => write!(f,"Network error" ),
            ConnectorError::InvalidAmount => write!(f, "Invalid amount"),
        }
    }
}

impl std::error::Error for ConnectorError {}

pub type ConnectorResult<T> = Result<T, ConnectorError>;

pub async fn select_connector(currency: &str) -> &'static str {
    match currency.to_uppercase().as_str() {
        "USD" | "EUR" | "GBP" => {
            info!("Selected Stripe connector for currency: {}", currency);
            "stripe"
        }
        _=> {
            info!("Selected Adyen connector for currency: {}", currency);
            "adyen"
        }
    }
}
