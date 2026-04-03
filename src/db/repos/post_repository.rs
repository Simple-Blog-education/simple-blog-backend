use crate::db::{db_connection::DbPool, repos::helpers::DieselRepository};

#[derive(Clone)]
pub struct PostRepository {
    pool: DbPool
}

impl DieselRepository for PostRepository {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl PostRepository {
    // db code here
}