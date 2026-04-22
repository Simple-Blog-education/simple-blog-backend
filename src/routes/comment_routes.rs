use rocket::{State, http::Status, serde::json::Json};
use uuid::Uuid;

use crate::{db::models::comment_models::{Comment, CommentChangeset, NewComment}, routes::jwt::Auth, services::{comment_service::CommentService, error::ServiceError}};



#[get("/posts/<post_id>/comments")]
pub async fn get_comments(post_id: Uuid, service: &State<CommentService>) -> Result<Json<Vec<Comment>>, (Status, Json<String>)> {
    match service.get_comments_by_post(post_id, 50).await {
        Ok(comments) => Ok(Json(comments)),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[get("/users/<user_id>/comments")]
pub async fn get_comments_user(user_id: Uuid, service: &State<CommentService>) -> Result<Json<Vec<Comment>>, (Status, Json<String>)> {
    match service.get_comments_by_user(user_id, 50).await {
        Ok(comments) => Ok(Json(comments)),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[put("/comments/<id>", format = "json", data = "<comment>")]
pub async fn put_comment(id: Uuid, comment: Json<CommentChangeset>, _jwt: Auth, service: &State<CommentService>) -> Result<Json<Comment>, (Status, Json<String>)> {
    match service.put_comment(id, comment.into_inner()).await {
        Ok(comment) => Ok(Json(comment)),
        Err(ServiceError::NotFound) => Err((Status::NotFound, format!("Comment with id {} not found", id).into())),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[post("/comments/new", format = "json", data = "<comment>")]
pub async fn post_comment(comment: Json<NewComment>, _jwt: Auth, service: &State<CommentService>) -> Result<Json<String>, (Status, Json<String>)> {
    match service.create_comment(comment.into_inner()).await {
        Ok(inserted) => Ok(Json(format!("Inserted {} comments", inserted).into())),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}

#[delete("/comments/<id>")]
pub async fn delete_comment(id: Uuid, _jwt: Auth, service: &State<CommentService>) -> Result<Status, (Status, Json<String>)> {
    match service.delete_comment(id).await {
        Ok(is_deleted) => match is_deleted {
            true => Ok(Status::NoContent),
            false => Err((Status::NotFound, Json(format!("Comment with id {} not found!", id))))
        },
        Err(e) => {
            eprintln!("Error loading user with id: {}: {}", id, e);
            Err((Status::InternalServerError, Json("Internal server error".into())))
        }
    }
}
