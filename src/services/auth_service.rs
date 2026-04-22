use crate::db::models::user_role::UserRole;
use crate::services::error::ServiceError;
use crate::routes::jwt::{Claims, JWT, TokenType, get_default_secret};
use crate::db::repos::user_repository::UserRepository;
use crate::db::models::user_models::{LoginCredentials, NewUser};

pub struct AuthService {
    repo: UserRepository
}

impl AuthService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn sign_up(&self, new_user: NewUser) -> Result<usize, ServiceError> {
        let inserted = self.repo.create_user(new_user)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::Internal)?;
        Ok(inserted)
    }

    pub async fn sign_in(&self, credentials: LoginCredentials) -> Result<String, ServiceError> {
        let user = self.repo.get_credentials_by_username(credentials.username)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        if user.password == credentials.password {
            let claims = Claims::new(user.username, TokenType::Auth);
            let token = JWT::make_token(&claims, get_default_secret()).map_err(ServiceError::from)?;
            Ok(token)
        }
        else {
            Err(ServiceError::Validation { reason: "Invalid password".to_string() })
        }
        
    }
    // check for role
    pub async fn check_user_role(&self, username: String, expected_role: UserRole) -> Result<bool, ServiceError> {
        let role = self.repo.get_role_by_username(username)
        .await
        .map_err(ServiceError::from)?;
        Ok(role.value() < expected_role.value())
    }

}