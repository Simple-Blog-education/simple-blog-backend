use bcrypt::BcryptError;

use crate::{db::repos::error::RepositoryError, routes::jwt::JWTError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Not found")]
    NotFound,
    #[error("Old password is incorrect")]
    InvalidOldPassword,
    #[error("Duplicate field: {field}")]
    Duplicate { field: String },
    #[error("Validation failed: {reason}")]
    Validation { reason: String },
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: RepositoryError,
    },
    #[error("JWT error")]
    JWT {
        #[from]
        source: JWTError,
    },
    #[error("Internal server error")]
    Internal,
    #[error("Password hashing error: {0}")]
    Bcrypt(#[from] BcryptError),
}
