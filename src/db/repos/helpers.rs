use std::future::Future;

use rocket::tokio::task::spawn_blocking;

use crate::db::{db_connection::DbPool, repos::error::RepositoryError};

pub trait DieselRepository {
    fn get_pool(&self) -> &DbPool;
    
    fn run_blocking<F, T>(&self, f: F) -> impl Future<Output = Result<T, RepositoryError>> + Send
where 
    F: FnOnce(&mut diesel::PgConnection) -> Result<T, RepositoryError> + Send + 'static,
    T: Send + 'static,
    {
        let pool = self.get_pool().clone();
        async move {
            let result = spawn_blocking(move || {
                let mut conn = pool.get()?;
                f(&mut conn)
            })
            .await
            .map_err(RepositoryError::from)??;
            Ok(result)
        }
    }
}

