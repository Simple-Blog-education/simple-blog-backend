use std::collections::HashMap;

use crate::db::models::like_models::PostLike;
use crate::db::{
    db_connection::DbPool,
    repos::{error::RepositoryError, helpers::DieselRepository},
};
use crate::schema::post_likes::dsl::post_likes;
use crate::schema::post_likes::{post_id as post_likes_post_id, user_id as post_likes_user_id};
use diesel::associations::HasTable;
use diesel::dsl::delete;
use diesel::{insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostLikeRepository {
    pool: DbPool,
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

    pub async fn get_likes_info(
        &self,
        post_ids: &[Uuid],
        current_user_id: Option<Uuid>,
    ) -> Result<HashMap<Uuid, (i64, bool)>, RepositoryError> {
        let ids = post_ids.to_vec();
        let uid = current_user_id;
        self.run_blocking(move |conn| {
            let counts: Vec<(Uuid, i64)> = post_likes::table()
                .filter(post_likes_post_id.eq_any(&ids))
                .group_by(post_likes_post_id)
                .select((post_likes_post_id, diesel::dsl::count_star()))
                .load(conn)?;

            let liked: HashMap<Uuid, bool> = if let Some(uid) = uid {
                post_likes::table()
                    .filter(post_likes_post_id.eq_any(&ids))
                    .filter(post_likes_user_id.eq(uid))
                    .select(post_likes_post_id)
                    .load::<Uuid>(conn)?
                    .into_iter()
                    .map(|pid| (pid, true))
                    .collect()
            } else {
                HashMap::new()
            };

            let mut result = HashMap::new();

            for (pid, count) in counts {
                let is_liked = liked.get(&pid).copied().unwrap_or(false);
                result.insert(pid, (count, is_liked));
            }

            for id in &ids {
                result.entry(*id).or_insert((0, false));
            }

            Ok(result)
        })
        .await
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
    pub async fn unlike(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, RepositoryError> {
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
        })
        .await
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
