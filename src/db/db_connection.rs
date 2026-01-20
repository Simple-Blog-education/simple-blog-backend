use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

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
