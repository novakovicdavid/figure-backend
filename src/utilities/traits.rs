use crate::server_errors::ServerError;

pub trait RandomNumberGenerator: Send + Sync {
    fn generate(&self) -> Result<u64, ServerError>;
}