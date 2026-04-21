use uuid::Uuid;
use crate::schema::comment_likes::comment_id as comment_likes_comment_id;
use crate::schema::comment_likes::dsl::comment_likes;
use crate::schema::comment_likes::user_id as comment_likes_user_id;
use diesel::dsl::delete;
use diesel::{
    insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use crate::db::{db_connection::DbPool, models::like_models::CommentLike, repos::{error::RepositoryError, helpers::DieselRepository}};


#[derive(Clone)]
pub struct CommentLikeRepository {
    pool: DbPool
}

impl DieselRepository for CommentLikeRepository {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl CommentLikeRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn like(&self, user_id: Uuid, comment_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let comment_like_struct = CommentLike {user_id, comment_id};
            let inserted_rows = insert_into(comment_likes)
                .values(comment_like_struct)
                .execute(conn)
                .map_err(RepositoryError::from)?;
            Ok(inserted_rows == 1)
        })
        .await
    }
    pub async fn unlike(&self, user_id: Uuid, comment_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = delete(
        comment_likes.filter(
            comment_likes_user_id
                .eq(user_id)
                .and(comment_likes_comment_id.eq(comment_id)),
                ),
            )
            .execute(conn)
            .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
    pub async fn is_liked(&self, user_id: Uuid, comment_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let likes = comment_likes
                .filter(comment_likes_user_id.eq(user_id))
                .filter(comment_likes_comment_id.eq(comment_id))
                .count()
                .first::<i64>(conn)
                .map_err(RepositoryError::from)?;
            Ok(likes == 1)
        })
        .await
    }
    pub async fn get_likes(&self, comment_id: Uuid) -> Result<i64, RepositoryError> {
        self.run_blocking(move |conn| {
            let count = comment_likes
                .filter(comment_likes_comment_id.eq(comment_id))
                .count()
                .first::<i64>(conn)
                .map_err(RepositoryError::from)?;
            Ok(count)
        })
        .await
    }
}