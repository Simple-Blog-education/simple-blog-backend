pub mod api;
pub mod db;
pub mod schema;
use std::env;
use crate::api::cors::cors_fairing;
use api::comment;
use api::index::index;
use api::like;
use api::post;
use api::user;
use api::auth;
use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::db_connection::init_pool(database_url.as_str());
    rocket::build()
        .manage(pool)
        .attach(cors_fairing())
        .mount(
            "/api/v1",
            routes![
                index,
                auth::user_new,
                auth::login,
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
                comment::post_comment,
                like::get_comment_likes,
                like::comment_is_liked_by_user,
                like::like_comment,
                like::delete_comment_like,
                like::get_post_likes,
                like::post_is_liked_by_user,
                like::like_post,
                like::delete_post_like
            ],
        )
}
