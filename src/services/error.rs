use thiserror::Error;

use crate::db::repos::error::RepositoryError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Not found")]
    NotFound,
    #[error("Duplicate field: {field}")]
    Duplicate { field: String },
    #[error("Validation failed: {reason}")]
    Validation {reason: String},
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: RepositoryError,
    },

    #[error("Internal server error")]
    Internal
}