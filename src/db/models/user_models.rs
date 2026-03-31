use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub reg_date: DateTime<Utc>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserChangeset {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String
}