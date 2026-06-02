use bcrypt::{hash, verify, DEFAULT_COST};

use crate::db::dto::user_dto::{AuthResponse, ChangePasswordRequest, SignInRequest, SignUpRequest};
use crate::db::models::user_models::{NewUser, User};
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

    pub async fn sign_up(&self, new_user: SignUpRequest) -> Result<User, ServiceError> {
        let new_user = NewUser::from(new_user);
        let hashed_password = hash(&new_user.password, DEFAULT_COST)?;
        let new_user_with_hash = NewUser {
            password: hashed_password,
            ..new_user
        };
        let user = self
            .repo
            .create_user(new_user_with_hash)
            .await
            .map_err(ServiceError::from)?;
        Ok(user)
    }

    pub async fn sign_in(&self, credentials: SignInRequest) -> Result<AuthResponse, ServiceError> {
        let user = self
            .repo
            .find_by_username(credentials.username.clone())
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        let password_matches = verify(credentials.password, &user.password_hash)?;
        if password_matches {
            let claims = Claims::new(user.username, user.id, TokenType::Auth);
            let token =
                JWT::make_token(&claims, get_default_secret()).map_err(ServiceError::from)?;
            let login_data = AuthResponse {
                user_id: user.id,
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
        let user = self
            .repo
            .find_by_username(username)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::Internal)?;
        let role = UserRole::from(user.role);
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
    pub async fn change_password(
        &self,
        username: String,
        changeset: ChangePasswordRequest,
    ) -> Result<bool, ServiceError> {
        let user = self
            .repo
            .find_by_username(username.clone())
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;

        let old_password_matches =
            verify(&changeset.old_password, &user.password_hash).map_err(ServiceError::Bcrypt)?;
        if !old_password_matches {
            return Err(ServiceError::InvalidOldPassword);
        }

        if changeset.old_password == changeset.new_password {
            return Err(ServiceError::Validation {
                reason: "New password must be different".into(),
            });
        }

        if changeset.new_password.len() < 8 {
            return Err(ServiceError::Validation {
                reason: "Password too short".into(),
            });
        }

        let new_hash = hash(changeset.new_password, DEFAULT_COST).map_err(ServiceError::Bcrypt)?;

        let updated = self
            .repo
            .update_password_hash(username, new_hash)
            .await
            .map_err(ServiceError::from)?;

        Ok(updated)
    }
}
