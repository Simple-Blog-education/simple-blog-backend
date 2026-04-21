use uuid::Uuid;
use crate::db::models::like_models::PostLike;
use crate::schema::post_likes::dsl::post_likes;
use crate::schema::post_likes::{post_id as post_likes_post_id, user_id as post_likes_user_id};
use diesel::dsl::delete;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, insert_into
};
use crate::db::{db_connection::DbPool, repos::{error::RepositoryError, helpers::DieselRepository}};

#[derive(Clone)]
pub struct PostLikeRepository {
    pool: DbPool
}

impl DieselRepository for PostLikeRepository {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl PostLikeRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn like(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let post_like_struct = PostLike { user_id, post_id };
            let inserted_rows = insert_into(post_likes)
            .values(post_like_struct)
            .execute(conn)
            .map_err(RepositoryError::from)?;
            Ok(inserted_rows == 1)
        })
        .await
    }
    pub async fn unlike (&self, user_id: Uuid, post_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = delete(
            post_likes.filter(
                post_likes_user_id
                    .eq(user_id)
                    .and(post_likes_post_id.eq(post_id)),
                ),
            )
            .execute(conn)
            .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        }).await
    }
    pub async fn is_liked(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let likes = post_likes
                .filter(post_likes_user_id.eq(user_id))
                .filter(post_likes_post_id.eq(post_id))
                .count()
                .first::<i64>(conn)
                .map_err(RepositoryError::from)?;
            Ok(likes == 1)
        })
        .await
    }
    pub async fn get_likes(&self, post_id: Uuid) -> Result<i64, RepositoryError> {
        self.run_blocking(move |conn| {
            let count = post_likes
                .filter(post_likes_post_id.eq(post_id))
                .count()
                .first::<i64>(conn)
                .map_err(RepositoryError::from)?;
            Ok(count)
        })
        .await
    }
}

