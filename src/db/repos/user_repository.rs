use crate::db::db_connection::DbPool;
use crate::db::models::user_models::{NewUser, User, UserProfileChangeset};
use crate::db::repos::error::RepositoryError;
use crate::db::repos::helpers::DieselRepository;
use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl DieselRepository for UserRepository {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(
        &self,
        username: String,
    ) -> Result<Option<User>, RepositoryError> {
        self.run_blocking(move |conn| {
            users::table
                .filter(users::username.eq(username))
                .select(User::as_select())
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError> {
        self.run_blocking(move |conn| {
            users::table
                .find(user_id)
                .select(User::as_select())
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn get_all_users(&self, limit: i64) -> Result<Vec<User>, RepositoryError> {
        self.run_blocking(move |conn| {
            users::table
                .select(User::as_select())
                .limit(limit)
                .load(conn)
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_select())
                .get_result(conn)
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        changeset: UserProfileChangeset,
    ) -> Result<User, RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::update(users::table.find(user_id))
                .set(changeset)
                .returning(User::as_select())
                .get_result(conn)
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn update_password_hash(
        &self,
        username: String,
        new_hash: String,
    ) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let updated = diesel::update(users::table.filter(users::username.eq(username)))
                .set(users::password.eq(new_hash))
                .execute(conn)?;
            Ok(updated == 1)
        })
        .await
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = diesel::delete(users::table.find(user_id)).execute(conn)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
}
