use crate::db::db_connection::DbPool;
use crate::db::models::post_models::{NewPost, Post, PostChangeset};
use crate::db::models::user_models::User;
use crate::db::pagination::Pagination;
use crate::db::repos::error::RepositoryError;
use crate::db::repos::helpers::DieselRepository;
use crate::schema::posts::dsl::posts;
use crate::schema::posts::{create_date, edit_date, header, text};
use crate::schema::users;
use diesel::dsl::{delete, now, update};
use diesel::{
    insert_into, BoolExpressionMethods, ExpressionMethods, OptionalExtension,
    PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostRepository {
    pool: DbPool,
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

    pub async fn search_posts(
        &self,
        pagination: Pagination,
        query: Option<String>,
    ) -> Result<(Vec<(Post, User)>, i64), RepositoryError> {
        let q = query.map(|s| s.to_string());
        self.run_blocking(move |conn| {
            let mut base_query = posts.into_boxed();
            if let Some(ref search) = q {
                let pattern = format!("%{}%", search);
                base_query = base_query.filter(
                    header
                        .ilike(pattern.clone())
                        .or(text.ilike(pattern.clone())),
                );
            }

            let total: i64 = posts.select(diesel::dsl::count_star()).first(conn)?;

            let items = base_query
                .inner_join(users::table)
                .select((Post::as_select(), User::as_select()))
                .order(create_date.desc())
                .limit(pagination.limit)
                .offset(pagination.offset)
                .load::<(Post, User)>(conn)?;

            Ok((items, total))
        })
        .await
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
        })
        .await
    }

    pub async fn put_post(
        &self,
        id: Uuid,
        changeset: PostChangeset,
    ) -> Result<Option<Post>, RepositoryError> {
        self.run_blocking(move |conn| {
            update(posts.find(id))
                .set((changeset, edit_date.eq(now)))
                .returning(Post::as_select())
                .get_result(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
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
