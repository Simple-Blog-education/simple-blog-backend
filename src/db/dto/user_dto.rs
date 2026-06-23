use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::user_models::User;

#[derive(FromForm)]
pub struct UserSearchParams {
    pub query: Option<String>,
    #[field(default = 1)]
    pub page: i64,
    #[field(default = 10)]
    pub per_page: i64,
}

// Запросы

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// Ответы

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub reg_date: DateTime<Utc>,
    pub role: String,
    pub avatar_url: Option<String>,
}

impl From<User> for UserProfileResponse {
    fn from(user: User) -> Self {
        UserProfileResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            reg_date: user.reg_date,
            role: user.role,
            avatar_url: user.avatar_url,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub token: String,
}
