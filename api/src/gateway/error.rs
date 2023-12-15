use crate::domain::error::DomainError;

impl From<sqlx::Error> for DomainError {
    fn from(error: sqlx::Error) -> Self {
        DomainError::GatewayError(anyhow::Error::new(error))
    }
}
