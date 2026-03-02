use diesel::{insert_into, RunQueryDsl};
use diesel::prelude::*;
use crate::api::jwt::{self, JWT};
use crate::db::models::{LoginCredentials, NewUser, TokenPair};
use rocket::serde::json::Json;
use crate::schema::users;
use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::schema::users::dsl::users as users_dsl;

#[post("/auth/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginCredentials>) -> Json<TokenPair> {
    let mut connection = PostgresConnection::new();
    let password = users::table.filter(users::password.eq(&data.password)).select((users::username, users::password)).first::<(String, String)>(&mut connection);
    let mut token_pair = TokenPair { auth_token: "".to_string(), refresh_token: "".to_string() };
    match password {
        Ok(user) => {
            let username = user.0;
            let payload = jwt::Payload::new(username,"admin".to_string(), jwt::TokenType::Auth);
            let token = JWT::make_token(jwt::DEFAULT_HEADER, payload, jwt::DEFAULT_SECRET.to_string()).unwrap();
            token_pair.auth_token = token;
        }
        Err(_) => return Json(token_pair)
    }
    return Json(token_pair);
}

#[post("/auth/signup", format = "json", data = "<data>")]
pub fn user_new(data: Json<NewUser<'_>>) -> String {
    let mut connection = PostgresConnection::new();
    let _ = insert_into(users_dsl)
        .values(data.into_inner())
        .execute(&mut connection)
        .expect("Error saving new user");
    "Success".to_string()
}

#[post("/auth/refresh")]
pub fn refresh() {

}