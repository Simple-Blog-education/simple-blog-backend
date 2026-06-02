use crate::db::models::comment_models::{Comment, CommentChangeset, NewComment};
use crate::db::pagination::Pagination;
use crate::db::repos::error::RepositoryError;
use crate::db::{db_connection::DbPool, repos::helpers::DieselRepository};
use crate::schema::{comments, users};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct CommentRepository {
    pool: DbPool,
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

    pub async fn get_comments(
        &self,
        post_id: Option<Uuid>,
        user_id: Option<Uuid>,
        pagination: Pagination,
    ) -> Result<(Vec<(Comment, String)>, i64), RepositoryError> {
        self.run_blocking(move |conn| {
            let mut query = comments::table
                .inner_join(users::table.on(comments::user_id.eq(users::id)))
                .into_boxed();

            if let Some(pid) = post_id {
                query = query.filter(comments::post_id.eq(pid));
            }
            if let Some(uid) = user_id {
                query = query.filter(comments::user_id.eq(uid));
            }

            let total: i64 = comments::table
                .inner_join(users::table.on(comments::user_id.eq(users::id)))
                .select(diesel::dsl::count_star())
                .first(conn)?;

            let items = query
                .select((Comment::as_select(), users::username))
                .order(comments::create_date.desc())
                .limit(pagination.limit)
                .offset(pagination.offset)
                .load::<(Comment, String)>(conn)?;

            Ok((items, total))
        })
        .await
    }
    pub async fn create_comment(&self, data: NewComment) -> Result<Option<usize>, RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::insert_into(comments::table)
                .values(data)
                .execute(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }
    pub async fn put_comment(
        &self,
        id: Uuid,
        changeset: CommentChangeset,
    ) -> Result<Option<Comment>, RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::update(comments::table.find(id))
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
            let deleted_rows = diesel::delete(comments::table.filter(comments::id.eq(comment_id)))
                .execute(conn)
                .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
}
