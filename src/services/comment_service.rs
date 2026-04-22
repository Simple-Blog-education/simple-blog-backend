use uuid::Uuid;

use crate::{db::{models::comment_models::{Comment, CommentChangeset, NewComment}, repos::comment_repository::CommentRepository}, services::error::ServiceError};

pub struct CommentService {
    repo: CommentRepository
}

impl CommentService {
    pub fn new(repo: CommentRepository) -> Self {
        Self { repo }
    }

    pub async fn get_comments_by_post(&self, post_id: Uuid, limit:usize) -> Result<Vec<Comment>, ServiceError> {
        let comments = self.repo.get_comments_by_post(post_id, limit)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        Ok(comments)
    }
    pub async fn get_comments_by_user(&self, user_id: Uuid, limit:usize) -> Result<Vec<Comment>, ServiceError> {
        let comments = self.repo.get_comments_by_user(user_id, limit)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        Ok(comments)
    }
    pub async fn create_comment(&self, data: NewComment) -> Result<usize, ServiceError> {
        let inserted_rows = self.repo.create_comment(data)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        Ok(inserted_rows)
    }
    pub async fn put_comment(&self, comment_id:Uuid, changeset: CommentChangeset) -> Result<Comment, ServiceError> {
        let changed = self.repo.put_comment(comment_id, changeset)
        .await
        .map_err(ServiceError::from)?
        .ok_or(ServiceError::NotFound)?;
        Ok(changed)
    }
    pub async fn delete_comment(&self, comment_id:Uuid) -> Result<bool, ServiceError> {
        let is_deleted = self.repo.delete_comment(comment_id)
        .await
        .map_err(ServiceError::from)?;
        Ok(is_deleted)
    }
}