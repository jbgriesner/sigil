use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Vault error: {0}")]
    Vault(#[from] serdevault::SerdeVaultError),

    #[error("Secret not found: {0}")]
    NotFound(Uuid),

    #[error("Password generator requires at least one character class")]
    EmptyCharset,

    #[error("Password length must be at least 1")]
    InvalidLength,
}
