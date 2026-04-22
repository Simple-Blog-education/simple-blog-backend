use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbPoolManager;

impl DbPoolManager {

    pub fn get_database_url_from_dotenv() -> String {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }

    pub fn init_pool(database_url: &str) -> DbPool {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool")
    }
}

