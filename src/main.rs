use crate::db::db_connection::{DBConnection, PostgresConnection};
use crate::db::models::{NewUser, User};
use bcrypt::DEFAULT_COST;
use diesel::dsl::insert_into;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use std::hash::Hash;

pub mod db;
pub mod schema;

fn main() {
    use schema::users::dsl::*;

    let mut connection = PostgresConnection::new();
    let password_hash = bcrypt::hash("example", DEFAULT_COST).unwrap();
    let _ = insert_into(users)
        .values(NewUser {
            username: "example",
            password: password_hash.as_str(),
            email: "example@mail.com",
            first_name: None,
            last_name: None,
        })
        .execute(&mut connection)
        .expect("Error creating user");
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users");
    for user in results {
        println!("{}", user.id);
    }
}
