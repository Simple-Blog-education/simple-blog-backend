use crate::db::models::user_models::{LoginCredentials, LoginData, NewUser, User};
use crate::routes::jwt::Auth;
use crate::services::auth_service::AuthService;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/auth/login", format = "json", data = "<data>")]
pub async fn sign_in(
    data: Json<LoginCredentials>,
    service: &State<AuthService>,
) -> Result<Json<LoginData>, (Status, Json<String>)> {
    match service.sign_in(data.into_inner()).await {
        Ok(token) => Ok(Json(token)),
        Err(e) => {
            eprintln!("Error while sign in: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal Server Error".to_string()),
            ))
        }
    }
}

#[post("/auth/signup", format = "json", data = "<data>")]
pub async fn sign_up(
    data: Json<NewUser>,
    service: &State<AuthService>,
) -> Result<Json<String>, (Status, Json<String>)> {
    match service.sign_up(data.0).await {
        Ok(inserted) => Ok(Json(format!("Inserted {} users", inserted))),
        Err(e) => {
            eprintln!("Error while sign up: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal Server Error".to_string()),
            ))
        }
    }
}

#[get("/auth/me", format = "json")]
pub async fn get_current_user(
    jwt: Auth,
    service: &State<AuthService>,
) -> Result<Json<User>, (Status, Json<String>)> {
    let username = jwt.0;
    match service.get_current_user(username).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            eprintln!("Error while fetching current user: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal Server Error".to_string()),
            ))
        }
    }
}
