use crate::db::models::user_models::{LoginCredentials, NewUser, UserChangeset};
use crate::db::models::user_role::UserRole;
use crate::db::repos::error::RepositoryError;
use crate::db::repos::helpers::DieselRepository;
use crate::db::{db_connection::DbPool, models::user_models::User};
use crate::schema::users::dsl::role as table_role;
use crate::schema::users::dsl::username as table_username;
use crate::schema::users::dsl::users;
use crate::schema::users::id;
use crate::schema::users::password as table_password;
use diesel::dsl::{delete, update};
use diesel::{insert_into, prelude::*};
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
            users
                .filter(table_username.eq(username))
                .select(User::as_select())
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn get_credentials_by_username(
        &self,
        username: String,
    ) -> Result<Option<LoginCredentials>, RepositoryError> {
        self.run_blocking(move |conn| {
            users
                .filter(table_username.eq(username))
                .select(LoginCredentials::as_select())
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn get_id_by_username(
        &self,
        username: String,
    ) -> Result<Option<Uuid>, RepositoryError> {
        self.run_blocking(move |conn| {
            users
                .filter(table_username.eq(username))
                .select(id)
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn get_role_by_username(
        &self,
        username: String,
    ) -> Result<UserRole, RepositoryError> {
        self.run_blocking(move |conn| {
            let role = users
                .filter(table_username.eq(username))
                .select(table_role)
                .first::<String>(conn)
                .optional()
                .map_err(RepositoryError::from)?;
            Ok(UserRole::from(role))
        })
        .await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError> {
        self.run_blocking(move |conn| {
            users
                .find(user_id)
                .select(User::as_select())
                .first(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn get_all_users(&self, limit: i64) -> Result<Option<Vec<User>>, RepositoryError> {
        self.run_blocking(move |conn| {
            users
                .select(User::as_select())
                .limit(limit)
                .load(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<Option<usize>, RepositoryError> {
        self.run_blocking(move |conn| {
            insert_into(users)
                .values(new_user)
                .execute(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn put_user(
        &self,
        user_id: Uuid,
        changeset: UserChangeset,
    ) -> Result<Option<User>, RepositoryError> {
        self.run_blocking(move |conn| {
            update(users.find(user_id))
                .set(changeset)
                .returning(User::as_select())
                .get_result(conn)
                .optional()
                .map_err(RepositoryError::from)
        })
        .await
    }

    pub async fn get_password_hash(
        &self,
        username: String,
    ) -> Result<Option<String>, RepositoryError> {
        self.run_blocking(move |conn| {
            let hash = users
                .filter(table_username.eq(username))
                .select(table_password)
                .first(conn)
                .optional()?;
            Ok(hash)
        })
        .await
    }

    pub async fn update_password_hash(
        &self,
        username: String,
        new_hash: String,
    ) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let updated = update(users.filter(table_username.eq(username)))
                .set(table_password.eq(new_hash))
                .execute(conn)?;
            Ok(updated == 1)
        })
        .await
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool, RepositoryError> {
        self.run_blocking(move |conn| {
            let deleted_rows = delete(users.find(user_id))
                .execute(conn)
                .map_err(RepositoryError::from)?;
            Ok(deleted_rows == 1)
        })
        .await
    }
}
