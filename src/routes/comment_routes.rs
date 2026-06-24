use rocket::{http::Status, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    db::{
        dto::{
            comment_dto::{CommentListParams, CommentResponse},
            PaginatedResponse,
        },
        models::comment_models::{Comment, CommentChangeset, NewComment},
    },
    routes::jwt::Auth,
    services::{comment_service::CommentService, error::ServiceError},
};

#[get("/comments?<params..>")]
pub async fn get_comments(
    params: CommentListParams,
    auth: Option<Auth>,
    service: &State<CommentService>,
) -> Result<Json<PaginatedResponse<CommentResponse>>, (Status, Json<String>)> {
    let current_user_id = auth.map(|a| a.user_id);
    match service
        .get_comments(
            params.post_id,
            params.user_id,
            params.page,
            params.per_page,
            current_user_id,
        )
        .await
    {
        Ok(comments) => Ok(Json(comments)),
        Err(e) => {
            let status = match e {
                ServiceError::Validation { .. } => Status::BadRequest,
                _ => Status::InternalServerError,
            };
            Err((status, Json(e.to_string())))
        }
    }
}

#[put("/comments/<id>", format = "json", data = "<comment>")]
pub async fn put_comment(
    id: Uuid,
    comment: Json<CommentChangeset>,
    auth: Auth,
    service: &State<CommentService>,
) -> Result<Json<Comment>, (Status, Json<String>)> {
    if auth.user_id != id && auth.role != "admin" {
        return Err((Status::Forbidden, Json("Access denied".into())));
    }
    match service.put_comment(id, comment.into_inner()).await {
        Ok(comment) => Ok(Json(comment)),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            format!("Comment with id {} not found", id).into(),
        )),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[post("/comments/new", format = "json", data = "<comment>")]
pub async fn post_comment(
    comment: Json<NewComment>,
    _auth: Auth,
    service: &State<CommentService>,
) -> Result<Json<String>, (Status, Json<String>)> {
    match service.create_comment(comment.into_inner()).await {
        Ok(inserted) => Ok(Json(format!("Inserted {} comments", inserted).into())),
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[delete("/comments/<id>")]
pub async fn delete_comment(
    id: Uuid,
    auth: Auth,
    service: &State<CommentService>,
) -> Result<Status, (Status, Json<String>)> {
    if auth.user_id != id && auth.role != "admin" {
        return Err((Status::Forbidden, Json("Access denied".into())));
    }
    match service.delete_comment(id).await {
        Ok(is_deleted) => match is_deleted {
            true => Ok(Status::NoContent),
            false => Err((
                Status::NotFound,
                Json(format!("Comment with id {} not found!", id)),
            )),
        },
        Err(e) => {
            eprintln!("Error loading user with id: {}: {}", id, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}
