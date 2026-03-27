use diesel::{insert_into, RunQueryDsl};
use diesel::prelude::*;
use crate::routes::jwt::{self, JWT};
use crate::db::models::user_models::{LoginCredentials, NewUser};
use rocket::serde::json::Json;
use crate::schema::users;
use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::schema::users::dsl::users as users_dsl;

#[post("/auth/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginCredentials>) -> Result<Json<String>, Json<String>> {
    let mut connection = PostgresConnection::new();
    let present = users::table.filter(users::username.eq(&data.username)).select((users::username, users::password, users::role)).first::<(String, String, String)>(&mut connection);
    let token;
    match present {
        Ok(user) => {
            let username = user.0;
            let role = user.2;
            let payload = jwt::Payload::new(username, role, jwt::TokenType::Auth);
            token = JWT::make_token(jwt::DEFAULT_HEADER, payload, jwt::get_default_secret()).unwrap();
        }
        Err(err) => {
            println!("{}", err);
            return Err(Json("Failed".to_string()));
        }
    }
    return Ok(Json(token));
}

#[post("/auth/signup", format = "json", data = "<data>")]
pub fn user_new(data: Json<NewUser<'_>>) -> Json<String> {
    let mut connection = PostgresConnection::new();
    let _ = insert_into(users_dsl)
        .values(data.into_inner())
        .execute(&mut connection)
        .expect("Error saving new user");
    Json("Success".to_string())
}