use diesel::data_types::PgDate;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub reg_date: PgDate
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub header: String,
    pub text: String,
    pub date: PgDate
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub user_id: &'a Uuid,
    pub header: &'a str,
    pub text: &'a str
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub text: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment<'a> {
    pub post_id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub text: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::post_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostLike {
    pub user_id: Uuid,
    pub post_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::post_likes)]
pub struct NewPostLike<'a> {
    pub user_id: &'a Uuid,
    pub post_id: &'a Uuid
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comment_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CommentLike {
    pub user_id: Uuid,
    pub comment_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comment_likes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCommentLike<'a> {
    pub comment_id: &'a Uuid,
    pub user_id: &'a Uuid,
}


