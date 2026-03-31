use diesel::dsl::{update, delete};
use rocket::tokio::task::spawn_blocking;

use crate::db::models::user_models::UserChangeset;
use crate::db::repos::error::RepositoryError;
use crate::db::{db_connection::DbPool, models::user_models::User};
use crate::schema::users::dsl::users;
use diesel::prelude::*;
use uuid::Uuid;
pub struct UserRepository {
    pool: DbPool
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
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


    pub async fn put_user(&self, user_id: Uuid, changeset: UserChangeset) -> Result<Option<User>, RepositoryError> {
        let pool = self.pool.clone();
        let result = spawn_blocking(move || -> Result<Option<User>, RepositoryError> {
            let mut conn = pool.get().expect("Db connection failed");
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