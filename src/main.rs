pub mod api;
pub mod db;
pub mod schema;

use api::index::index;
use api::user;
use api::post;
use api::comment;
use api::like;
use crate::api::cors::CORS;

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
        comment::post_comment,
        like::get_comment_likes,
        like::comment_is_liked_by_user,
        like::like_comment,
        like::delete_comment_like,
        like::get_post_likes,
        like::post_is_liked_by_user,
        like::like_post,
        like::delete_post_like
    ]).attach(CORS)
}
