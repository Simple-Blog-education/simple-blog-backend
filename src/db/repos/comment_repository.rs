use uuid::Uuid;
use crate::db::repos::error::RepositoryError;
use crate::db::models::comment_models::{Comment, CommentChangeset, NewComment};
use crate::schema::comments::dsl::comments;
use crate::schema::comments::{
    id as comments_id, post_id as comments_post_id, user_id as comments_user_id,
};
use diesel::dsl::{insert_into, update};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, delete};
use crate::db::{db_connection::DbPool, repos::helpers::DieselRepository};

#[derive(Clone)]
pub struct CommentRepository {
    pool: DbPool
}

impl DieselRepository for CommentRepository {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl CommentRepository {

    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_comments_by_post(&self, post_id: Uuid, limit: usize) -> Result<Option<Vec<Comment>>, RepositoryError> {
        self.run_blocking(move |conn| {
            comments
            .limit(limit as i64)
            .filter(comments_post_id.eq(post_id))
            .select(Comment::as_select())
            .load(conn)
            .optional()
            .map_err(RepositoryError::from)
        })
        .await
    }
    pub async fn get_comments_by_user(&self, user_id: Uuid, limit: usize) -> Result<Option<Vec<Comment>>, RepositoryError> {
        self.run_blocking(move |conn| {
            comments
            .limit(limit as i64)
            .filter(comments_user_id.eq(user_id))
            .select(Comment::as_select())
            .load(conn)
            .optional()
            .map_err(RepositoryError::from)
        })
        .await
    }
    pub async fn create_comment(&self, data: NewComment) -> Result<Option<usize>, RepositoryError> {
        self.run_blocking(move |conn| {
            insert_into(comments)
            .values(data)
            .execute(conn)
            .optional()
            .map_err(RepositoryError::from)
        })
        .await
    }
    pub async fn put_comment(&self, id:Uuid, changeset: CommentChangeset) -> Result<Option<Comment>, RepositoryError> {
        self.run_blocking(move |conn| {
            update(comments.find(id))
            .set(changeset)
            .returning(Comment::as_select())
            .get_result(conn)
            .optional()
            .map_err(RepositoryError::from)
        })
        .await
    }
    pub async fn delete_comment(&self, comment_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = delete(comments.filter(comments_id.eq(comment_id)))
            .execute(conn)
            .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
}