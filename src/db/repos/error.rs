use diesel::result::Error as DieselError;
use diesel::r2d2::PoolError as PoolError;
use rocket::tokio::task::JoinError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Diesel(#[from] DieselError),
    #[error("Connection pool error: {0}")]
    Pool(#[from] PoolError),
    #[error("Task join error: {0}")]
    Join(#[from] JoinError)
}