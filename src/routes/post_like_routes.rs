use crate::routes::jwt::Auth;
use crate::services::error::ServiceError;
use crate::services::post_like_service::PostLikeService;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

#[get("/users/id/<user_id>/post_likes/<post_id>")]
pub async fn post_is_liked_by_user(
    user_id: Uuid,
    post_id: Uuid,
    service: &State<PostLikeService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    match service.is_liked(user_id, post_id).await {
        Ok(success) => Ok(Json(success)),
        Err(e) => {
            eprintln!("Error loading like: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[post("/users/<user_id>/post_likes/<post_id>")]
pub async fn like_post(
    user_id: Uuid,
    post_id: Uuid,
    _jwt: Auth,
    service: &State<PostLikeService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    match service.like(user_id, post_id).await {
        Ok(success) => Ok(Json(success)),
        Err(e) => {
            eprintln!("Error liking post: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[delete("/users/<user_id>/post_likes/<post_id>")]
pub async fn unlike_post(
    user_id: Uuid,
    post_id: Uuid,
    _jwt: Auth,
    service: &State<PostLikeService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    match service.unlike(user_id, post_id).await {
        Ok(success) => Ok(Json(success)),
        Err(ServiceError::NotFound) => {
            Err((Status::NotFound, Json(format!("Like not found!").into())))
        }
        Err(e) => {
            eprintln!("Error liking post: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}
