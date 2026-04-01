use diesel::dsl::{update, delete};
use rocket::tokio::task::spawn_blocking;

use crate::db::models::user_models::{LoginCredentials, NewUser, UserChangeset};
use crate::db::models::user_role::UserRole;
use crate::db::repos::error::RepositoryError;
use crate::db::{db_connection::DbPool, models::user_models::User};
use crate::schema::users::dsl::users;
use crate::schema::users::dsl::username as table_username;
use crate::schema::users::dsl::role as table_role;
use diesel::{insert_into, prelude::*};
use uuid::Uuid;
pub struct UserRepository {
    pool: DbPool
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(&self, username: String) -> Result<Option<User>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<User>, RepositoryError> {
            let mut conn = pool.get()?;
            let user = users.filter(table_username.eq(username))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;
            Ok(user)
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn get_credentials_by_username(&self, username: String) -> Result<Option<LoginCredentials>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<LoginCredentials>, RepositoryError> {
            let mut conn = pool.get()?;
            let user = users.filter(table_username.eq(username))
            .select(LoginCredentials::as_select())
            .first(&mut conn)
            .optional()?;
            Ok(user)
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn get_role_by_username(&self, username: String) -> Result<UserRole, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<UserRole, RepositoryError> {
            let mut conn = pool.get()?;
            let role = users.filter(table_username.eq(username))
            .select(table_role)
            .first::<String>(&mut conn)
            .optional()?;
            Ok(UserRole::from(role))
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<User>, RepositoryError> {
            let mut conn = pool.get()?;
            let user = users.find(user_id)
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;
            Ok(user)
            
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn get_all_users(&self, limit: i64) -> Result<Option<Vec<User>>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<Vec<User>>, RepositoryError> {
            let mut conn = pool.get()?;
            let users_struct = users
            .select(User::as_select())
            .limit(limit)
            .load(&mut conn)
            .optional()?;
            Ok(users_struct)
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<Option<usize>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<usize>, RepositoryError> {
            let mut conn = pool.get()?;
            let inserted: Option<usize> = insert_into(users)
        .values(new_user)
        .execute(&mut conn)
        .optional()?;
        Ok(inserted)
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn put_user(&self, user_id: Uuid, changeset: UserChangeset) -> Result<Option<User>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<User>, RepositoryError> {
            let mut conn = pool.get()?;
            let updated = update(users.find(user_id))
        .set(changeset)
        .returning(User::as_select())
        .get_result(&mut conn)
        .optional()?;
        Ok(updated)
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<Option<String>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<String>, RepositoryError> {
            let mut conn = pool.get()?;
            let deleted_rows = delete(users.find(user_id))
            .execute(&mut conn)?;
            match deleted_rows == 1 {
                // господи, что это... TODO: поменять возвращаемое значение
                true => Ok(Some("OK".to_owned())),
                false => Err(RepositoryError::Delete { reason: "Duplicate delete!".to_owned() })
            }
        })
        .await
        .map_err(RepositoryError::from)??;
        Ok(result)
    }
}