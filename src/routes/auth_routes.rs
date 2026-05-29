use crate::db::dto::user_dto::{
    AuthResponse, ChangePasswordRequest, SignInRequest, SignUpRequest, UserProfileResponse,
};
use crate::routes::jwt::Auth;
use crate::services::auth_service::AuthService;
use crate::services::error::ServiceError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/auth/login", format = "json", data = "<data>")]
pub async fn sign_in(
    data: Json<SignInRequest>,
    service: &State<AuthService>,
) -> Result<Json<AuthResponse>, (Status, Json<String>)> {
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
    data: Json<SignUpRequest>,
    service: &State<AuthService>,
) -> Result<Json<UserProfileResponse>, (Status, Json<String>)> {
    match service.sign_up(data.0).await {
        Ok(user) => Ok(Json(UserProfileResponse::from(user))),
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
) -> Result<Json<UserProfileResponse>, (Status, Json<String>)> {
    let username = jwt.0;
    match service.get_current_user(username).await {
        Ok(user) => Ok(Json(UserProfileResponse::from(user))),
        Err(e) => {
            eprintln!("Error while fetching current user: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal Server Error".to_string()),
            ))
        }
    }
}

#[put("/auth/change_password", format = "json", data = "<data>")]
pub async fn change_password(
    data: Json<ChangePasswordRequest>,
    jwt: Auth,
    service: &State<AuthService>,
) -> Result<Json<bool>, (Status, Json<String>)> {
    let username = jwt.0;
    match service.change_password(username, data.0).await {
        Ok(success) => Ok(Json(success)),
        Err(ServiceError::InvalidOldPassword) => Err((
            Status::Unauthorized,
            Json("Old password is incorrect".into()),
        )),
        Err(ServiceError::Validation { reason }) => Err((Status::BadRequest, Json(reason))),
        Err(ServiceError::NotFound) => Err((Status::NotFound, Json("User not found".into()))),
        Err(e) => {
            eprintln!("Error changing password: {}", e);
            Err((
                Status::InternalServerError,
                Json("Internal server error".into()),
            ))
        }
    }
}
