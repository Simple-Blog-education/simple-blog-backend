use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::Roletype"]
pub enum Role {
    Admin,
    User,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub reg_date: DateTime<Utc>,
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

#[derive(Deserialize)]
pub struct NewPasswordChangeset {
    pub old_password: String,
    pub new_password: String,
}

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
pub struct NewComment<'a> {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub text: &'a str,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct CommentChangeset {
    pub text: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::post_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostLike {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::post_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPostLike {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comment_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommentLike {
    pub user_id: Uuid,
    pub comment_id: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::comment_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCommentLike {
    pub comment_id: Uuid,
    pub user_id: Uuid,
}
