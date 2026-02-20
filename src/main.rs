pub mod api;
pub mod db;
pub mod schema;

use crate::api::cors::CORS;
use crate::api::jwt::Header;
use crate::api::jwt::Payload;
use api::comment;
use api::index::index;
use api::like;
use api::post;
use api::user;
use api::jwt;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let token = jwt::JWT::make_token(Header::new("HS256".to_owned(), "JWT".to_owned()),
        Payload::new("lol".to_owned(), "Admin".to_owned(), jwt::TokenType::Auth),
        jwt::DEFAULT_SECRET.to_owned());
    print!("{}", token.unwrap());
    rocket::build()
        .mount(
            "/api/v1",
            routes![
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
            ],
        )
        .attach(CORS)
}
