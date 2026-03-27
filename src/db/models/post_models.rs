use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub header: String,
    pub text: String,
    pub create_date: DateTime<Utc>,
    pub edit_date: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub user_id: Uuid,
    pub header: &'a str,
    pub text: &'a str,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostChangeset {
    pub header: String,
    pub text: String,
}