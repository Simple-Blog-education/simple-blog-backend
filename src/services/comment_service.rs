use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    db::{
        dto::{comment_dto::CommentResponse, PaginatedResponse},
        models::comment_models::{Comment, CommentChangeset, NewComment},
        pagination::Pagination,
        repos::{
            comment_like_repository::CommentLikeRepository, comment_repository::CommentRepository,
        },
    },
    services::error::ServiceError,
};

pub struct CommentService {
    repo: CommentRepository,
    like_repo: CommentLikeRepository,
}

impl CommentService {
    pub fn new(repo: CommentRepository, like_repo: CommentLikeRepository) -> Self {
        Self { repo, like_repo }
    }

    pub async fn get_comments(
        &self,
        post_id: Option<Uuid>,
        user_id: Option<Uuid>,
        page: i64,
        per_page: i64,
        current_user_id: Option<Uuid>,
    ) -> Result<PaginatedResponse<CommentResponse>, ServiceError> {
        let pagination_db = Pagination::new(page, per_page, 50).map_err(ServiceError::from)?;

        let (comments_with_username, total) = self
            .repo
            .get_comments(post_id, user_id, pagination_db)
            .await?;

        let comment_ids: Vec<Uuid> = comments_with_username.iter().map(|(c, _)| c.id).collect();

        let likes_info: HashMap<Uuid, (i64, bool)> = self
            .like_repo
            .get_likes_info(&comment_ids, current_user_id)
            .await?;

        let data = comments_with_username
            .into_iter()
            .map(|(comment, username)| {
                let (likes, is_liked) = likes_info.get(&comment.id).copied().unwrap_or((0, false));

                CommentResponse {
                    id: comment.id,
                    post_id: comment.post_id,
                    username,
                    text: comment.text,
                    create_date: comment.create_date,
                    likes,
                    is_liked,
                }
            })
            .collect();

        Ok(PaginatedResponse {
            data,
            total,
            page,
            per_page,
        })
    }
    pub async fn create_comment(&self, data: NewComment) -> Result<usize, ServiceError> {
        let inserted_rows = self
            .repo
            .create_comment(data)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(inserted_rows)
    }
    pub async fn put_comment(
        &self,
        comment_id: Uuid,
        changeset: CommentChangeset,
    ) -> Result<Comment, ServiceError> {
        let changed = self
            .repo
            .put_comment(comment_id, changeset)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(changed)
    }
    pub async fn delete_comment(&self, comment_id: Uuid) -> Result<bool, ServiceError> {
        let is_deleted = self
            .repo
            .delete_comment(comment_id)
            .await
            .map_err(ServiceError::from)?;
        Ok(is_deleted)
    }
}
