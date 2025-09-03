use tracing::info;

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