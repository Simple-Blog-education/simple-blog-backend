use crate::db::models::user_models::{LoginCredentials, LoginData, NewUser, User};
use crate::db::models::user_role::UserRole;
use crate::db::repos::user_repository::UserRepository;
use crate::routes::jwt::{get_default_secret, Claims, TokenType, JWT};
use crate::services::error::ServiceError;

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn sign_up(&self, new_user: NewUser) -> Result<usize, ServiceError> {
        let inserted = self
            .repo
            .create_user(new_user)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::Internal)?;
        Ok(inserted)
    }

    pub async fn sign_in(&self, credentials: LoginCredentials) -> Result<LoginData, ServiceError> {
        let user = self
            .repo
            .get_credentials_by_username(credentials.username.clone())
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        if user.password == credentials.password {
            let id = self
                .repo
                .get_id_by_username(credentials.username)
                .await
                .map_err(ServiceError::from)?
                .ok_or(ServiceError::NotFound)?;
            let claims = Claims::new(user.username, TokenType::Auth);
            let token =
                JWT::make_token(&claims, get_default_secret()).map_err(ServiceError::from)?;
            let login_data = LoginData {
                user_id: id.to_string(),
                token: token,
            };
            Ok(login_data)
        } else {
            Err(ServiceError::Validation {
                reason: "Invalid password".to_string(),
            })
        }
    }
    // check for role
    pub async fn check_user_role(
        &self,
        username: String,
        expected_role: UserRole,
    ) -> Result<bool, ServiceError> {
        let role = self
            .repo
            .get_role_by_username(username)
            .await
            .map_err(ServiceError::from)?;
        Ok(role.value() < expected_role.value())
    }

    pub async fn get_current_user(&self, username: String) -> Result<User, ServiceError> {
        let user = self
            .repo
            .find_by_username(username)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(user)
    }
}
