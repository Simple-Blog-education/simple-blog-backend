use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::db::models::{Comment, CommentChangeset, NewComment};
use crate::schema::comments::dsl::comments;
use crate::schema::comments::{
    id as comments_id, post_id as comments_post_id, user_id as comments_user_id,
};
use diesel::dsl::{insert_into, update};
use diesel::{delete, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/posts/<post_id>/comments")]
pub fn get_comments(post_id: Uuid) -> Option<Json<Vec<Comment>>> {
    let mut connection = PostgresConnection::new();
    let comments_result = comments
        .limit(50)
        .filter(comments_post_id.eq(post_id))
        .load(&mut connection)
        .expect("Error loading comments");
    Some(Json(comments_result))
}

#[get("/users/<user_id>/comments")]
pub fn get_comments_user(user_id: Uuid) -> Json<Vec<Comment>> {
    let mut connection = PostgresConnection::new();
    let comments_result = comments
        .limit(50)
        .filter(comments_user_id.eq(user_id))
        .load(&mut connection)
        .expect("Error loading comments");
    Json(comments_result)
}

#[put("/comments/<id>", format = "json", data = "<comment>")]
pub fn put_comment(id: Uuid, comment: Json<CommentChangeset>) -> Option<Json<Comment>> {
    let mut connection = PostgresConnection::new();
    let result = update(comments.find(id))
        .set(comment.into_inner())
        .returning(Comment::as_select())
        .get_result(&mut connection)
        .expect("Error updating comment");
    Some(Json(result))
}

#[post("/comments/new", format = "json", data = "<comment>")]
pub fn post_comment(comment: Json<NewComment<'_>>) -> Json<String> {
    let mut connection = PostgresConnection::new();
    let _ = insert_into(comments)
        .values(comment.into_inner())
        .execute(&mut connection)
        .expect("Error saving comment");
    Json("Success".to_string())
}

#[delete("/comments/<id>")]
pub fn delete_comment(id: Uuid) -> Option<Json<String>> {
    let mut connection = PostgresConnection::new();
    let _ = delete(comments.filter(comments_id.eq(id))).execute(&mut connection);
    Some(Json("Success".to_string()))
}
