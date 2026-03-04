use crate::api::jwt::JWT;
use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::db::models::{User, UserChangeset};
use crate::schema::users::dsl::users;
use diesel::dsl::{delete, update};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/users/all")]
pub fn user_all(_jwt: JWT) -> Json<Vec<User>> {
    let mut connection = PostgresConnection::new();
    let result = users
        .limit(500)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users");
    Json(result)
}
#[get("/users/<id>")]
pub fn get_user(id: Uuid) -> Result<Json<User>, Json<String>> {
    let mut connection = PostgresConnection::new();
    let user = users
        .find(id)
        .select(User::as_select())
        .first(&mut connection)
        .expect("Error loading user");
    Ok(Json(user))
}

#[put("/users/<id>", format = "json", data = "<data>")]
pub fn put_user(id: Uuid, data: Json<UserChangeset>, _token: JWT) -> Json<String> {
    let mut connection = PostgresConnection::new();
    let _ = update(users.find(id))
        .set(data.into_inner())
        .execute(&mut connection)
        .expect("Error updating user");
    Json(String::from("Success"))
}

#[delete("/users/<id>")]
pub fn delete_user(id: Uuid, _jwt: JWT) -> Result<Json<String>, Json<String>> {
    let mut connection = PostgresConnection::new();
    let _ = delete(users.find(id))
        .execute(&mut connection)
        .expect("Error deleting user");
    Ok(Json("Success".to_string()))
}
