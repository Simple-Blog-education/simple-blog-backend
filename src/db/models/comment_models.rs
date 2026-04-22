use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub text: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewComment {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub text: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct CommentChangeset {
    pub text: String,
}