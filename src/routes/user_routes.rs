use crate::db::models::user_models::{User, UserChangeset};
use crate::routes::jwt::Auth;
use crate::services::error::ServiceError;
use crate::services::user_service::UserService;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

#[get("/users/all")]
pub async fn user_all(
    _jwt: Auth,
    service: &State<UserService>,
) -> Result<Json<Vec<User>>, (Status, Json<String>)> {
    match service.get_all_users(500).await {
        Ok(users_struct) => Ok(Json(users_struct)),
        Err(e) => {
            eprintln!("Error loading users: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}
#[get("/users/<id>")]
pub async fn get_user_by_id(
    id: Uuid,
    service: &State<UserService>,
) -> Result<Json<User>, (Status, Json<String>)> {
    match service.get_user_by_id(id).await {
        Ok(user) => Ok(Json(user)),
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

#[get("/users?<username>")]
pub async fn get_user_by_username(
    username: String,
    service: &State<UserService>,
) -> Result<Json<User>, (Status, Json<String>)> {
    match service.get_user_by_username(username.clone()).await {
        Ok(user) => Ok(Json(user)),
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
    data: Json<UserChangeset>,
    _token: Auth,
    service: &State<UserService>,
) -> Result<(Status, Json<User>), (Status, Json<String>)> {
    match service.put_user(id, data.into_inner()).await {
        Ok(changed) => Ok((Status::Created, Json(changed))),
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
