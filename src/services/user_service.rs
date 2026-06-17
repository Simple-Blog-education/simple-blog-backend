use uuid::Uuid;

use crate::{
    db::{
        dto::{
            user_dto::{UpdateProfileRequest, UserProfileResponse},
            PaginatedResponse,
        },
        models::user_models::{User, UserProfileChangeset},
        pagination::Pagination,
        repos::user_repository::UserRepository,
    },
    services::error::ServiceError,
};

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, ServiceError> {
        let user = self
            .repo
            .find_by_id(user_id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: String) -> Result<User, ServiceError> {
        let user = self
            .repo
            .find_by_username(username)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(user)
    }

    pub async fn search_users(
        &self,
        page: i64,
        per_page: i64,
        query: Option<String>,
    ) -> Result<PaginatedResponse<UserProfileResponse>, ServiceError> {
        let pagination = Pagination::new(page, per_page, 100).map_err(ServiceError::from)?;
        let (users, total) = self
            .repo
            .search_users(pagination, query)
            .await
            .map_err(ServiceError::from)?;
        let data = users
            .into_iter()
            .map(|user| -> UserProfileResponse { UserProfileResponse::from(user) })
            .collect();
        Ok(PaginatedResponse {
            data,
            total,
            page,
            per_page,
        })
    }

    pub async fn get_all_users(&self, limit: i64) -> Result<Vec<User>, ServiceError> {
        let users = self
            .repo
            .get_all_users(limit)
            .await
            .map_err(ServiceError::from)?;
        Ok(users)
    }

    pub async fn put_user(
        &self,
        user_id: Uuid,
        changeset: UpdateProfileRequest,
    ) -> Result<User, ServiceError> {
        let model = UserProfileChangeset::from(changeset);
        let updated = self
            .repo
            .update_profile(user_id, model)
            .await
            .map_err(ServiceError::from)?;
        Ok(updated)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool, ServiceError> {
        let success = self
            .repo
            .delete_user(user_id)
            .await
            .map_err(ServiceError::from)?;
        Ok(success)
    }
}
