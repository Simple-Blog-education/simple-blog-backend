use crate::db::db_connection::DbPool;
use crate::db::models::post_models::{NewPost, Post, PostChangeset};
use crate::db::repos::error::RepositoryError;
use crate::db::repos::helpers::DieselRepository;
use crate::schema::posts::dsl::posts;
use crate::schema::posts::edit_date;
use diesel::dsl::{delete, now, update};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, insert_into};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostRepository {
    pool: DbPool
}

impl DieselRepository for PostRepository {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl PostRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_posts(&self, limit: usize) -> Result<Option<Vec<Post>>, RepositoryError> {
        self.run_blocking(move |conn| {
            posts
            .limit(limit as i64)
            .select(Post::as_select())
            .load(conn)
            .optional()
            .map_err(RepositoryError::from)
        }).await
    }

    pub async fn get_post_by_id(&self, id: Uuid) -> Result<Option<Post>, RepositoryError> {
        self.run_blocking(move |conn| {
            posts
            .find(id)
            .select(Post::as_select())
            .first(conn)
            .optional()
            .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn create_post(&self, new_post: NewPost) -> Result<Option<usize>, RepositoryError> {
        self.run_blocking(move |conn| {
            insert_into(posts)
            .values(new_post)
            .execute(conn)
            .optional()
            .map_err(RepositoryError::from)
        }).await
    }

    pub async fn put_post(&self, id:Uuid, changeset: PostChangeset) -> Result<Option<Post>, RepositoryError> {
        self.run_blocking(move |conn| {
            update(posts.find(id))
            .set((changeset, edit_date.eq(now)))
            .returning(Post::as_select())
            .get_result(conn)
            .optional()
            .map_err(RepositoryError::from)
        }).await
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = delete(posts.find(id))
            .execute(conn)
            .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
}