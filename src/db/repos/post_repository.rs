use crate::{
    db::{
        db_connection::DbPool,
        models::{
            post_models::{NewPost, Post, PostChangeset},
            user_models::User,
        },
        pagination::Pagination,
        repos::{error::RepositoryError, helpers::DieselRepository},
    },
    schema::{posts, users},
};
use diesel::prelude::*;
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
            let mut base_query = posts::table.into_boxed();
            if let Some(ref search) = q {
                let pattern = format!("%{}%", search);
                base_query = base_query.filter(
                    posts::header
                        .ilike(pattern.clone())
                        .or(posts::text.ilike(pattern.clone())),
                );
            }

            let total: i64 = posts::table.select(diesel::dsl::count_star()).first(conn)?;

            let items = base_query
                .inner_join(users::table)
                .select((Post::as_select(), User::as_select()))
                .order(posts::create_date.desc())
                .limit(pagination.limit)
                .offset(pagination.offset)
                .load::<(Post, User)>(conn)?;

            Ok((items, total))
        })
        .await
    }

    pub async fn get_post_by_id(
        &self,
        post_id: Uuid,
    ) -> Result<Option<(Post, User)>, RepositoryError> {
        self.run_blocking(move |conn| {
            posts::table
                .inner_join(users::table)
                .filter(posts::id.eq(post_id))
                .select((Post::as_select(), User::as_select()))
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn create_post(&self, new_post: NewPost) -> Result<Option<usize>, RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::insert_into(posts::table)
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
            diesel::update(posts::table.find(id))
                .set((changeset, posts::edit_date.eq(diesel::dsl::now)))
                .returning(Post::as_select())
                .get_result(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = diesel::delete(posts::table.find(id))
                .execute(conn)
                .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
}
