use crate::db::dto::user_dto::{UpdateProfileRequest, UserProfileResponse, UserSearchParams};
use crate::db::dto::PaginatedResponse;
use crate::routes::jwt::Auth;
use crate::services::error::ServiceError;
use crate::services::user_service::UserService;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

#[get("/users?<params..>")]
pub async fn search_users(
    service: &State<UserService>,
    params: UserSearchParams,
) -> Result<Json<PaginatedResponse<UserProfileResponse>>, (Status, Json<String>)> {
    match service
        .search_users(params.page, params.per_page, params.query)
        .await
    {
        Ok(users) => Ok(Json(users)),
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

#[get("/users/id/<id>")]
pub async fn get_user_by_id(
    id: Uuid,
    service: &State<UserService>,
) -> Result<Json<UserProfileResponse>, (Status, Json<String>)> {
    match service.get_user_by_id(id).await {
        Ok(user) => Ok(Json(UserProfileResponse::from(user))),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            Json(format!("User with id {} not found", id).into()),
        )),
        Err(e) => {
            eprintln!("Error loading user with id: {}: {}", id, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[get("/users/username/<username>")]
pub async fn get_user_by_username(
    username: String,
    service: &State<UserService>,
) -> Result<Json<UserProfileResponse>, (Status, Json<String>)> {
    match service.get_user_by_username(username.clone()).await {
        Ok(user) => Ok(Json(UserProfileResponse::from(user))),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            Json(format!("User with username {} not found", username.clone()).into()),
        )),
        Err(e) => {
            eprintln!("Error loading user with username: {}: {}", username, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[put("/users/<id>", format = "json", data = "<data>")]
pub async fn put_user(
    id: Uuid,
    data: Json<UpdateProfileRequest>,
    _token: Auth,
    service: &State<UserService>,
) -> Result<(Status, Json<UserProfileResponse>), (Status, Json<String>)> {
    match service.put_user(id, data.into_inner()).await {
        Ok(user) => Ok((Status::Created, Json(UserProfileResponse::from(user)))),
        Err(ServiceError::NotFound) => Err((
            Status::NotFound,
            Json(format!("User with id {} not found", id).into()),
        )),
        Err(e) => {
            eprintln!("Error loading user with id: {}: {}", id, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}

#[delete("/users/<id>")]
pub async fn delete_user(
    id: Uuid,
    _jwt: Auth,
    service: &State<UserService>,
) -> Result<Status, (Status, Json<String>)> {
    match service.delete_user(id).await {
        Ok(msg) => match msg {
            true => Ok(Status::NoContent),
            false => Err((
                Status::NotFound,
                Json(format!("User with id {} not found", id).into()),
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

#[derive(FromForm)]
pub struct AvatarUpload<'f> {
    pub file: TempFile<'f>,
}

#[post("/users/me/avatar", data = "<upload>")]
pub async fn upload_avatar(
    upload: Form<AvatarUpload<'_>>,
    auth: Auth,
    service: &State<UserService>,
) -> Result<Json<UserProfileResponse>, (Status, Json<String>)> {
    let file = upload.into_inner().file;
    match service.update_avatar(auth.1, file).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            eprintln!("Error uploading avatar for user with id: {}: {}", auth.1, e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}
