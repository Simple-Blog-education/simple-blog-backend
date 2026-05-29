use crate::routes::jwt::Auth;
use crate::services::comment_like_service::CommentLikeService;
use crate::services::error::ServiceError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

#[get("/likes/comments/<comment_id>")]
pub async fn get_comment_likes(
    comment_id: Uuid,
    service: &State<CommentLikeService>,
) -> Result<Json<i64>, (Status, Json<String>)> {
    match service.get_likes(comment_id).await {
        Ok(likes) => Ok(Json(likes)),
        Err(ServiceError::NotFound) => {
            Err((Status::NotFound, Json(format!("Comment not found!").into())))
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

#[get("/users/id/<user_id>/comment_likes/<comment_id>")]
pub async fn comment_is_liked_by_user(
    user_id: Uuid,
    comment_id: Uuid,
    service: &State<CommentLikeService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    match service.is_liked(user_id, comment_id).await {
        Ok(success) => Ok(Json(success)),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            Json(format!("User or comment not found!").into()),
        )),
        Err(e) => {
            eprintln!("Error liking post: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[post("/users/<user_id>/comment_likes/<comment_id>")]
pub async fn like_comment(
    user_id: Uuid,
    comment_id: Uuid,
    _jwt: Auth,
    service: &State<CommentLikeService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    match service.like(user_id, comment_id).await {
        Ok(success) => Ok(Json(success)),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            Json(format!("User or comment not found!").into()),
        )),
        Err(e) => {
            eprintln!("Error liking post: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[delete("/users/<user_id>/comment_likes/<comment_id>")]
pub async fn unlike_comment(
    user_id: Uuid,
    comment_id: Uuid,
    _jwt: Auth,
    service: &State<CommentLikeService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    match service.unlike(user_id, comment_id).await {
        Ok(success) => Ok(Json(success)),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            Json(format!("User or comment not found!").into()),
        )),
        Err(e) => {
            eprintln!("Error liking post: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}
