use uuid::Uuid;

use crate::{
    db::{
        dto::{post_dto::PostResponse, PaginatedResponse},
        models::post_models::{NewPost, Post, PostChangeset},
        pagination::Pagination,
        repos::post_repository::PostRepository,
    },
    services::error::ServiceError,
};

pub struct PostService {
    repo: PostRepository,
}

impl PostService {
    pub fn new(repo: PostRepository) -> Self {
        Self { repo }
    }

    pub async fn search_posts(
        &self,
        page: i64,
        per_page: i64,
        query: Option<String>,
    ) -> Result<PaginatedResponse<PostResponse>, ServiceError> {
        let pagination = Pagination::new(page, per_page, 100).map_err(ServiceError::from)?;
        let (posts_db, total) = self
            .repo
            .search_posts(pagination, query)
            .await
            .map_err(ServiceError::from)?;

        let data = posts_db
            .into_iter()
            .map(|(p, u)| -> PostResponse { PostResponse::from_post(p, u) })
            .collect();

        Ok(PaginatedResponse {
            data: data,
            total,
            page,
            per_page,
        })
    }

    pub async fn get_post_by_id(&self, id: Uuid) -> Result<Post, ServiceError> {
        let post = self
            .repo
            .get_post_by_id(id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound)?;
        Ok(post)
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
