pub mod db;
pub mod routes;
pub mod schema;
pub mod services;
use routes::cors::cors_fairing;
use services::{auth_service::AuthService, post_service::PostService, user_service::UserService};
use db::repos::{post_repository::PostRepository, user_repository::UserRepository};
use routes::{auth_routes, comment_routes, index::index, like_routes, post_routes, user_routes};

use crate::{db::repos::comment_repository::CommentRepository, services::comment_service::CommentService};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let database_url = db::db_connection::DbPoolManager::get_database_url_from_dotenv();
    let pool = db::db_connection::DbPoolManager::init_pool(database_url.as_str());

    let user_repo = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repo.clone());
    let auth_service = AuthService::new(user_repo);

    let post_repo = PostRepository::new(pool.clone());
    let post_service = PostService::new(post_repo);

    let comment_repo = CommentRepository::new(pool.clone());
    let comment_service = CommentService::new(comment_repo);

    rocket::build()
    .manage(pool)
    .manage(user_service)
    .manage(auth_service)
    .manage(post_service)
    .manage(comment_service)
    .attach(cors_fairing())
    .mount(
        "/api/v1",
        routes![
            index,
            auth_routes::sign_in,
            auth_routes::sign_up,
            user_routes::get_user,
            user_routes::user_all,
            user_routes::delete_user,
            user_routes::put_user,
            post_routes::get_post_by_id,
            post_routes::get_all_posts,
            post_routes::create_post,
            post_routes::put_post,
            post_routes::delete_post,
            comment_routes::get_comments,
            comment_routes::get_comments_user,
            comment_routes::delete_comment,
            comment_routes::put_comment,
            comment_routes::post_comment,
            like_routes::get_comment_likes,
            like_routes::comment_is_liked_by_user,
            like_routes::like_comment,
            like_routes::delete_comment_like,
            like_routes::get_post_likes,
            like_routes::post_is_liked_by_user,
            like_routes::like_post,
            like_routes::delete_post_like
        ],
    )
}
