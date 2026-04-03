use crate::db::models::post_models::{NewPost, Post, PostChangeset};
use crate::routes::jwt::Auth;
use crate::services::error::ServiceError;
use crate::services::post_service::PostService;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/posts/all")]
pub async fn get_all_posts(service: &State<PostService>) -> Result<Json<Vec<Post>>, (Status, Json<String>)> {
    match service.get_all_posts(50).await {
        Ok(posts) => Ok(Json(posts)),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[get("/posts/<id>")]
pub async fn get_post_by_id(id: Uuid, service: &State<PostService>) -> Result<Json<Post>, (Status, Json<String>)> {
    match service.get_post_by_id(id).await {
        Ok(post) => Ok(Json(post)),
        Err(ServiceError::NotFound) => Err((Status::NotFound, format!("Post with id {} not found", id).into())),
        Err(e) => {
            eprintln!("Error loading post with id {}: {}", id, e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[post("/posts/new", format = "json", data = "<post>")]
pub async fn create_post(post: Json<NewPost>, _jwt: Auth, service: &State<PostService>) -> Result<Json<String>, (Status, Json<String>)> {
    match service.create_post(post.into_inner()).await {
        Ok(inserted) => Ok(Json(format!("Inserted {} posts", inserted).into())),
        Err(e) => {
            eprintln!("Error creating post: {}", e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[put("/posts/<id>", format = "json", data = "<post>")]
pub async fn put_post(id: Uuid, post: Json<PostChangeset>, _jwt: Auth, service: &State<PostService>) -> Result<Json<Post>, (Status, Json<String>)> {
    match service.put_post(id, post.into_inner()).await {
        Ok(changed_post) => Ok(Json(changed_post)),
        Err(ServiceError::NotFound) => Err((Status::NotFound, format!("Post with id {} not found", id).into())),
        Err(e) => {
            eprintln!("Error editing post with id {}: {}", id, e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[delete("/posts/<id>")]
pub async fn delete_post(id: Uuid, _jwt: Auth, service: &State<PostService>) -> Result<Status, (Status, Json<String>)> {
    match service.delete_post(id).await {
        Ok(is_deleted) => match is_deleted {
            true => Ok(Status::NoContent),
            false => Err((Status::NotFound, Json(format!("Post with id {} not found!", id))))
        },
        Err(e) => {
            eprintln!("Error loading user with id: {}: {}", id, e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}
