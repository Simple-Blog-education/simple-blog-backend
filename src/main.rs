pub mod api;
pub mod db;
pub mod schema;
use api::index::index;
use api::user;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![index, user::user_new, user::get_user, user::user_all])
}
