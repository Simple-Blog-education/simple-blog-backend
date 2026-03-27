use diesel::{Connection, PgConnection, r2d2::{self, ConnectionManager}};
use dotenvy::dotenv;
use std::env;

// DEPRECATED
pub trait DBConnection<Connection> {
    fn new() -> Connection;
}

pub struct PostgresConnection;

impl DBConnection<PgConnection> for PostgresConnection {
    fn new() -> PgConnection {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
    }
}

// USE THIS!!!

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

