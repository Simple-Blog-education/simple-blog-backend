use uuid::Uuid;

use crate::{db::repos::comment_like_repository::CommentLikeRepository, services::error::ServiceError};



pub struct CommentLikeService {
    repo: CommentLikeRepository
}

impl CommentLikeService {
    pub fn new(repo: CommentLikeRepository) -> Self {
        Self { repo }
    }

    pub async fn like(&self, user_id: Uuid, comment_id: Uuid) -> Result<bool, ServiceError> {
        let success = self.repo.like(user_id, comment_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(success)
    }
    pub async fn unlike(&self, user_id: Uuid, comment_id: Uuid) -> Result<bool, ServiceError> {
        let success = self.repo.unlike(user_id, comment_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(success)
    }
    pub async fn is_liked(&self, user_id: Uuid, comment_id: Uuid)  -> Result<bool, ServiceError> {
        let success = self.repo.is_liked(user_id, comment_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(success)
    }
    pub async fn get_likes(&self, comment_id: Uuid) -> Result<i64, ServiceError> {
        let likes = self.repo.get_likes(comment_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(likes)
    }
}