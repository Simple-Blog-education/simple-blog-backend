use std::str::FromStr;
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::dsl::update;
use dotenvy::Iter;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::db::models::{NewUser, User};
use crate::schema::users::dsl::users;

#[post("/users/new", format = "json", data = "<data>")]
pub fn user_new(data: Json<NewUser<'_>>) -> String {
    let mut connection = PostgresConnection::new();
    let _ = insert_into(users).values(data.into_inner()).execute(&mut connection).expect("Error saving new user");
    "Success".to_string()
}

#[get("/users/all")]
pub fn user_all() -> Json<Vec<User>> {
    let mut connection = PostgresConnection::new();
    let result = users.limit(500).select(User::as_select()).load(&mut connection).expect("Error loading users");
    Json(result)
}
#[get("/users/<id>")]
pub fn get_user(id: &str) -> Result<Json<User>, Json<String>> {
    let uuid = match Uuid::from_str(id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(Json(e.to_string())),
    };
    let mut connection = PostgresConnection::new();
    let user = users.find(uuid).select(User::as_select()).first(&mut connection).expect("Error loading user");
    Ok(Json(user))
}

#[put("/users/<id>", format = "json", data = "<data>")]
pub fn put_user(id: &str, data: Json<User>) -> Result<Json<User>, Json<String>> {
    let uuid = match Uuid::from_str(id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(Json(e.to_string())),
    };
    let mut connection = PostgresConnection::new();
    let user = update(users.find(uuid)).set(data.into_inner()).get_result(&mut connection).expect("Error updating user");
    Ok(Json(user))
}