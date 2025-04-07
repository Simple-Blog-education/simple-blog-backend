pub mod api;
pub mod db;
pub mod schema;

use diesel::sql_types::Json;
use rocket::Config;
use rocket::config::LogLevel;
use rocket::figment::value::magic::RelativePathBuf;
use rocket::http::ContentType;
use api::index::index;
use api::user;
use api::post;
use api::comment;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![
        index,
        user::user_new,
        user::get_user,
        user::user_all,
        user::delete_user,
        user::put_user,
        post::get_post,
        post::get_all_posts,
        post::post_post,
        post::put_post,
        post::delete_post,
        comment::get_comments,
        comment::get_comments_user,
        comment::delete_comment,
        comment::put_comment,
        comment::post_comment
    ])
}
