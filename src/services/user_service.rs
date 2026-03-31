use uuid::Uuid;

use crate::{db::{models::user_models::{User, UserChangeset}, repos::user_repository::UserRepository}, services::error::ServiceError};

pub struct UserService {
    repo: UserRepository
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, ServiceError> {
        let user = self.repo.find_by_id(user_id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(user)
    }

    pub async fn get_all_users(&self, limit: i64) -> Result<Vec<User>, ServiceError> {
        let users = self.repo.get_all_users(limit)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(users)
    }

    pub async fn put_user(&self, user_id: Uuid, changeset: UserChangeset) -> Result<User, ServiceError> {
        let updated = self.repo.put_user(user_id, changeset)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        Ok(updated)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<String, ServiceError> {
        let success = self.repo.delete_user(user_id)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        Ok(success)
    }
}