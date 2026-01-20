use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::db::models::{NewPost, Post, PostChangeset};
use crate::schema::posts::dsl::posts;
use crate::schema::posts::edit_date;
use diesel::dsl::{delete, now, update};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/posts/all")]
pub fn get_all_posts() -> Json<Vec<Post>> {
    let mut connection = PostgresConnection::new();
    let posts_result = posts
        .limit(50)
        .select(Post::as_select())
        .load(&mut connection)
        .expect("Error loading posts");
    Json(posts_result)
}

#[get("/posts/<id>")]
pub fn get_post(id: Uuid) -> Option<Json<Post>> {
    let mut connection = PostgresConnection::new();
    let post_result = posts
        .find(id)
        .select(Post::as_select())
        .first(&mut connection)
        .expect("Error loading post");
    Some(Json(post_result))
}

#[post("/posts/new", format = "json", data = "<post>")]
pub fn post_post(post: Json<NewPost<'_>>) -> String {
    let mut connection = PostgresConnection::new();
    let _ = insert_into(posts)
        .values(post.into_inner())
        .execute(&mut connection)
        .expect("Error saving post");
    "Success".to_string()
}

#[put("/posts/<id>", format = "json", data = "<post>")]
pub fn put_post(id: Uuid, post: Json<PostChangeset>) -> Option<Json<Post>> {
    let mut connection = PostgresConnection::new();
    let updated_post = update(posts.find(id))
        .set((post.into_inner(), edit_date.eq(now)))
        .returning(Post::as_select())
        .get_result(&mut connection)
        .expect("Error updating post");
    Some(Json(updated_post))
}

#[delete("/posts/<id>")]
pub fn delete_post(id: Uuid) -> Option<String> {
    let mut connection = PostgresConnection::new();
    let _ = delete(posts.find(id))
        .execute(&mut connection)
        .expect("Error deleting post");
    Some("Success".to_string())
}
