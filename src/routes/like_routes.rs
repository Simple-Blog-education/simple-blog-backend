use crate::routes::jwt::Auth;
use crate::routes::user_routes::get_user;
use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::db::models::like_models::{CommentLike, NewCommentLike, PostLike};
use crate::db::models::user_models::User;
use crate::schema::comment_likes::comment_id as comment_likes_comment_id;
use crate::schema::comment_likes::dsl::comment_likes;
use crate::schema::comment_likes::user_id as comment_likes_user_id;
use crate::schema::post_likes::dsl::post_likes;
use crate::schema::post_likes::{post_id as post_likes_post_id, user_id as post_likes_user_id};
use diesel::dsl::delete;
use diesel::{
    insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/likes/comments/<comment_id>")]
pub fn get_comment_likes(comment_id: Uuid) -> Json<Vec<User>> {
    let mut connection = PostgresConnection::new();
    let likes = comment_likes
        .select(CommentLike::as_select())
        .filter(comment_likes_comment_id.eq(comment_id))
        .load(&mut connection)
        .expect("Error loading likes");
    let mut result: Vec<User> = vec![];
    for like in likes.iter() {
        // result.push(get_user(like.user_id).unwrap().into_inner());
    }
    Json(result)
}

#[get("/likes/posts/<post_id>")]
pub fn get_post_likes(post_id: Uuid) -> Json<Vec<User>> {
    let mut connection = PostgresConnection::new();
    let likes = post_likes
        .select(PostLike::as_select())
        .filter(post_likes_post_id.eq(post_id))
        .load(&mut connection)
        .expect("Error loading likes");
    let mut result: Vec<User> = vec![];
    for like in likes.iter() {
        // result.push(get_user(like.user_id).unwrap().into_inner());
    }
    Json(result)
}

#[get("/users/<user_id>/post_likes/<post_id>")]
pub fn post_is_liked_by_user(user_id: Uuid, post_id: Uuid) -> Json<bool> {
    let mut connection = PostgresConnection::new();
    let liked = post_likes
        .select(PostLike::as_select())
        .filter(
            post_likes_user_id
                .eq(user_id)
                .and(post_likes_post_id.eq(post_id)),
        )
        .load(&mut connection)
        .expect("Error loading likes");
    if liked.is_empty() {
        return Json(false);
    }
    Json(true)
}

#[post("/users/<user_id>/post_likes/<post_id>")]
pub fn like_post(user_id: Uuid, post_id: Uuid, _jwt: Auth) -> Option<Json<String>> {
    let mut connection = PostgresConnection::new();
    let post_like_struct = PostLike { user_id, post_id };
    let _ = insert_into(post_likes)
        .values(post_like_struct)
        .execute(&mut connection)
        .expect("Error updating post");
    Some(Json("Success".to_string()))
}

#[delete("/users/<user_id>/post_likes/<post_id>")]
pub fn delete_post_like(user_id: Uuid, post_id: Uuid, _jwt: Auth) -> Option<Json<String>> {
    let mut connection = PostgresConnection::new();
    let _ = delete(
        post_likes.filter(
            post_likes_user_id
                .eq(user_id)
                .and(post_likes_post_id.eq(post_id)),
        ),
    )
    .execute(&mut connection)
    .expect("Error deleting post");
    Some(Json("Success".to_string()))
}

#[get("/users/<user_id>/comment_likes/<comment_id>")]
pub fn comment_is_liked_by_user(user_id: Uuid, comment_id: Uuid) -> Json<bool> {
    let mut connection = PostgresConnection::new();
    let liked = comment_likes
        .select(CommentLike::as_select())
        .filter(
            comment_likes_user_id
                .eq(user_id)
                .and(comment_likes_comment_id.eq(comment_id)),
        )
        .load(&mut connection)
        .expect("Error loading likes");
    if liked.is_empty() {
        return Json(false);
    }
    Json(true)
}

#[post("/users/<user_id>/comment_likes/<comment_id>")]
pub fn like_comment(user_id: Uuid, comment_id: Uuid, _jwt: Auth) -> Option<Json<String>> {
    let mut connection = PostgresConnection::new();
    let comment_like_struct = NewCommentLike {
        user_id,
        comment_id,
    };
    let _ = insert_into(comment_likes)
        .values(comment_like_struct)
        .execute(&mut connection)
        .expect("Error liking comment");
    Some(Json("Success".to_string()))
}

#[delete("/users/<user_id>/comment_likes/<comment_id>")]
pub fn delete_comment_like(user_id: Uuid, comment_id: Uuid, _jwt: Auth) -> Option<Json<String>> {
    let mut connection = PostgresConnection::new();
    let _ = delete(
        comment_likes.filter(
            comment_likes_user_id
                .eq(user_id)
                .and(comment_likes_comment_id.eq(comment_id)),
        ),
    )
    .execute(&mut connection)
    .expect("Error deleting post");
    Some(Json("Success".to_string()))
}
