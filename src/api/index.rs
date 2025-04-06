use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::form::Form;
use rocket::serde::json::Json;
use crate::db::db_connection::{PostgresConnection, DBConnection};
use crate::db::models::{User};
use crate::schema::users::dsl::users;

#[get("/")]
pub fn index() -> &'static str {
    "Health check!"
}

