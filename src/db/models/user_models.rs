use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::dto::user_dto::{SignUpRequest, UpdateProfileRequest};

#[derive(Queryable, Selectable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[diesel(column_name = password)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub reg_date: DateTime<Utc>,
    pub role: String,
    pub avatar_url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<SignUpRequest> for NewUser {
    fn from(req: SignUpRequest) -> Self {
        NewUser {
            username: req.username,
            password: req.password,
            email: req.email,
            first_name: req.first_name,
            last_name: req.last_name,
        }
    }
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfileChangeset {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

impl From<UpdateProfileRequest> for UserProfileChangeset {
    fn from(req: UpdateProfileRequest) -> Self {
        UserProfileChangeset {
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
        }
    }
}
