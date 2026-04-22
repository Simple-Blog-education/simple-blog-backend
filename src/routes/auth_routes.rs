use rocket::State;
use rocket::http::Status;
use crate::db::models::user_models::{LoginCredentials, NewUser};
use crate::services::auth_service::AuthService;
use rocket::serde::json::Json;

#[post("/auth/login", format = "json", data = "<data>")]
pub async fn sign_in(data: Json<LoginCredentials>, service: &State<AuthService>) -> Result<Json<String>, (Status, Json<String>)> {
    match service.sign_in(data.into_inner()).await {
        Ok(token) => Ok(Json(token)),
        Err(e) => {
            eprintln!("Error while sign in: {}", e);
            Err((Status::InternalServerError, Json("Internal Server Error".to_string())))
        }
    }
}

#[post("/auth/signup", format = "json", data = "<data>")]
pub async fn sign_up(data: Json<NewUser>, service: &State<AuthService>) -> Result<Json<String>, (Status, Json<String>)> {
    match service.sign_up(data.0).await {
        Ok(inserted) => Ok(Json(format!("Inserted {} users", inserted))),
        Err(e) => {
            eprintln!("Error while sign up: {}", e);
            Err((Status::InternalServerError, Json("Internal Server Error".to_string())))
        }
    }
}