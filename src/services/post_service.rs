use uuid::Uuid;

use crate::{
    db::{
        dto::{post_dto::PostResponse, PaginatedResponse},
        models::post_models::{NewPost, Post, PostChangeset},
        pagination::Pagination,
        repos::{post_like_repository::PostLikeRepository, post_repository::PostRepository},
    },
    services::error::ServiceError,
};

pub struct PostService {
    repo: PostRepository,
    like_repo: PostLikeRepository,
}

impl PostService {
    pub fn new(repo: PostRepository, like_repo: PostLikeRepository) -> Self {
        Self { repo, like_repo }
    }

    pub async fn search_posts(
        &self,
        page: i64,
        per_page: i64,
        query: Option<String>,
        current_user_id: Option<Uuid>,
    ) -> Result<PaginatedResponse<PostResponse>, ServiceError> {
        let pagination = Pagination::new(page, per_page, 100).map_err(ServiceError::from)?;
        let (posts_with_users, total) = self
            .repo
            .search_posts(pagination, query)
            .await
            .map_err(ServiceError::from)?;

        let post_ids: Vec<Uuid> = posts_with_users.iter().map(|(p, _)| p.id).collect();
        let likes_info = self
            .like_repo
            .get_likes_info(&post_ids, current_user_id)
            .await?;

        let data = posts_with_users
            .into_iter()
            .map(|(post, user)| {
                let (likes, is_liked) = likes_info.get(&post.id).copied().unwrap_or((0, false));
                PostResponse::from_post_and_user(post, user, likes, is_liked)
            })
            .collect();

        Ok(PaginatedResponse {
            data: data,
            total,
            page,
            per_page,
        })
    }

    pub async fn get_post_by_id(
        &self,
        id: Uuid,
        current_user_id: Option<Uuid>,
    ) -> Result<PostResponse, ServiceError> {
        let (post, user) = self
            .repo
            .get_post_by_id(id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        let likes_info = self
            .like_repo
            .get_likes_info(&[id], current_user_id)
            .await?;
        let (likes, is_liked) = likes_info.get(&id).copied().unwrap_or((0, false));

        Ok(PostResponse::from_post_and_user(
            post, user, likes, is_liked,
        ))
    }

    pub async fn create_post(&self, post: NewPost) -> Result<usize, ServiceError> {
        let created_rows = self
            .repo
            .create_post(post)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(created_rows)
    }

    pub async fn put_post(&self, id: Uuid, changeset: PostChangeset) -> Result<Post, ServiceError> {
        let changed = self
            .repo
            .put_post(id, changeset)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(changed)
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<bool, ServiceError> {
        let is_deleted = self
            .repo
            .delete_post(id)
            .await
            .map_err(ServiceError::from)?;
        Ok(is_deleted)
    }
}
