use uuid::Uuid;

use crate::{db::repos::post_like_repository::PostLikeRepository, services::error::ServiceError};

pub struct PostLikeService {
    repo: PostLikeRepository
}

impl PostLikeService {
    pub fn new(repo: PostLikeRepository) -> Self {
        Self { repo }
    }

    pub async fn like(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, ServiceError> {
        let success = self.repo.like(user_id, post_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(success)
    }

    pub async fn unlike(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, ServiceError> {
        let success = self.repo.unlike(user_id, post_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(success)
    }

    pub async fn is_liked(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, ServiceError> {
        let success = self.repo.is_liked(user_id, post_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(success)
    }

    pub async fn get_likes(&self, post_id: Uuid) -> Result<i64, ServiceError> {
        let likes = self.repo.get_likes(post_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(likes)
    }
}