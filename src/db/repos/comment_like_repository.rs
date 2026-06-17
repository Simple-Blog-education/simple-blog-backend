use std::collections::HashMap;

use crate::db::{
    db_connection::DbPool,
    models::like_models::CommentLike,
    repos::{error::RepositoryError, helpers::DieselRepository},
};
use crate::schema::comment_likes::comment_id as comment_likes_comment_id;
use crate::schema::comment_likes::dsl::comment_likes;
use crate::schema::comment_likes::user_id as comment_likes_user_id;
use diesel::dsl::delete;
use diesel::{insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone)]
pub struct CommentLikeRepository {
    pool: DbPool,
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

    pub async fn get_likes_info(
        &self,
        comment_ids: &[Uuid],
        current_user_id: Option<Uuid>,
    ) -> Result<HashMap<Uuid, (i64, bool)>, RepositoryError> {
        let ids = comment_ids.to_vec();
        let uid = current_user_id;
        self.run_blocking(move |conn| {
            // Запрос 1: общее количество лайков для каждого comment_id
            let counts = comment_likes
                .filter(comment_likes_comment_id.eq_any(&ids))
                .group_by(comment_likes_comment_id)
                .select((comment_likes_comment_id, diesel::dsl::count_star()))
                .load::<(Uuid, i64)>(conn)?;

            // Запрос 2: для тех же comment_id проверяем, есть ли лайк от конкретного пользователя
            let liked_map = if let Some(uid) = uid {
                comment_likes
                    .filter(comment_likes_comment_id.eq_any(&ids))
                    .filter(comment_likes_user_id.eq(uid))
                    .group_by(comment_likes_comment_id)
                    .select((comment_likes_comment_id, diesel::dsl::count_star()))
                    .load::<(Uuid, i64)>(conn)?
                    .into_iter()
                    .map(|(cid, cnt)| (cid, cnt > 0))
                    .collect::<HashMap<Uuid, bool>>()
            } else {
                HashMap::new()
            };

            // Собираем результат
            let mut result = HashMap::new();
            for (cid, count) in counts {
                let liked = liked_map.get(&cid).copied().unwrap_or(false);
                result.insert(cid, (count, liked));
            }
            // Добавляем комментарии без лайков (им count=0, liked=false)
            for id in &ids {
                result.entry(*id).or_insert((0, false));
            }
            Ok(result)
        })
        .await
    }

    pub async fn like(&self, user_id: Uuid, comment_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let comment_like_struct = CommentLike {
                user_id,
                comment_id,
            };
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
