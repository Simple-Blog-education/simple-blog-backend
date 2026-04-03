use uuid::Uuid;

use crate::{db::{models::post_models::{NewPost, Post, PostChangeset}, repos::post_repository::PostRepository}, services::error::ServiceError};

pub struct PostService {
    repo: PostRepository
}

impl PostService {
    pub fn new(repo: PostRepository) -> Self {
        Self { repo }
    }
    
    pub async fn get_all_posts(&self, limit: usize) -> Result<Vec<Post>, ServiceError> {
        let posts = self.repo.get_all_posts(limit)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(posts)
    }

    pub async fn get_post_by_id(&self, id: Uuid) -> Result<Post, ServiceError> {
        let post = self.repo.get_post_by_id(id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(post)
    }

    pub async fn create_post(&self, post: NewPost) -> Result<usize, ServiceError> {
        let created_rows = self.repo.create_post(post)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(created_rows)
    }

    pub async fn put_post(&self, id: Uuid, changeset: PostChangeset) -> Result<Post, ServiceError> {
        let changed = self.repo.put_post(id, changeset)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(changed)
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<bool, ServiceError> {
        let is_deleted = self.repo.delete_post(id)
            .await
            .map_err(ServiceError::from)?;
        Ok(is_deleted)
    }
}