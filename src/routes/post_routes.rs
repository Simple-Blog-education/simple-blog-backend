use crate::db::dto::post_dto::{PostResponse, PostSearchParams};
use crate::db::dto::PaginatedResponse;
use crate::db::models::post_models::{NewPost, Post, PostChangeset};
use crate::routes::jwt::Auth;
use crate::services::error::ServiceError;
use crate::services::post_service::PostService;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

#[get("/posts?<params..>")]
pub async fn search_posts(
    service: &State<PostService>,
    params: PostSearchParams,
) -> Result<Json<PaginatedResponse<PostResponse>>, (Status, Json<String>)> {
    match service
        .search_posts(params.page, params.per_page, params.query)
        .await
    {
        Ok(posts) => Ok(Json(posts)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            let status = match e {
                ServiceError::Validation { .. } => Status::BadRequest,
                _ => Status::InternalServerError,
            };
            Err((status, Json(e.to_string())))
        }
    }
}

#[get("/posts/<id>")]
pub async fn get_post_by_id(
    id: Uuid,
    service: &State<PostService>,
) -> Result<Json<Post>, (Status, Json<String>)> {
    match service.get_post_by_id(id).await {
        Ok(post) => Ok(Json(post)),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            format!("Post with id {} not found", id).into(),
        )),
        Err(e) => {
            eprintln!("Error loading post with id {}: {}", id, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[post("/posts/new", format = "json", data = "<post>")]
pub async fn create_post(
    post: Json<NewPost>,
    _jwt: Auth,
    service: &State<PostService>,
) -> Result<Json<String>, (Status, Json<String>)> {
    match service.create_post(post.into_inner()).await {
        Ok(inserted) => Ok(Json(format!("Inserted {} posts", inserted).into())),
        Err(e) => {
            eprintln!("Error creating post: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[put("/posts/<id>", format = "json", data = "<post>")]
pub async fn put_post(
    id: Uuid,
    post: Json<PostChangeset>,
    _jwt: Auth,
    service: &State<PostService>,
) -> Result<Json<Post>, (Status, Json<String>)> {
    match service.put_post(id, post.into_inner()).await {
        Ok(changed_post) => Ok(Json(changed_post)),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            format!("Post with id {} not found", id).into(),
        )),
        Err(e) => {
            eprintln!("Error editing post with id {}: {}", id, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[delete("/posts/<id>")]
pub async fn delete_post(
    id: Uuid,
    _jwt: Auth,
    service: &State<PostService>,
) -> Result<Status, (Status, Json<String>)> {
    match service.delete_post(id).await {
        Ok(is_deleted) => match is_deleted {
            true => Ok(Status::NoContent),
            false => Err((
                Status::NotFound,
                Json(format!("Post with id {} not found!", id)),
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
