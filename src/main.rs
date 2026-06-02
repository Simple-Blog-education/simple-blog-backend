pub mod db;
pub mod routes;
pub mod schema;
pub mod services;
use db::repos::{post_repository::PostRepository, user_repository::UserRepository};
use routes::cors::cors_fairing;
use routes::{
    auth_routes, comment_like_routes, comment_routes, index::index, post_like_routes, post_routes,
    user_routes,
};
use services::{auth_service::AuthService, post_service::PostService, user_service::UserService};

use crate::{
    db::repos::{
        comment_like_repository::CommentLikeRepository, comment_repository::CommentRepository,
        post_like_repository::PostLikeRepository,
    },
    services::{
        comment_like_service::CommentLikeService, comment_service::CommentService,
        post_like_service::PostLikeService,
    },
};

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

    let post_like_repo = PostLikeRepository::new(pool.clone());
    let post_like_service = PostLikeService::new(post_like_repo);

    let comment_like_repo = CommentLikeRepository::new(pool.clone());
    let comment_like_service = CommentLikeService::new(comment_like_repo.clone());

    let comment_service = CommentService::new(comment_repo, comment_like_repo);

    rocket::build()
        .manage(pool)
        .manage(user_service)
        .manage(auth_service)
        .manage(post_service)
        .manage(comment_service)
        .manage(post_like_service)
        .manage(comment_like_service)
        .attach(cors_fairing())
        .mount(
            "/api/v1",
            routes![
                index,
                auth_routes::sign_in,
                auth_routes::sign_up,
                auth_routes::get_current_user,
                auth_routes::change_password,
                user_routes::get_user_by_id,
                user_routes::get_user_by_username,
                user_routes::search_users,
                user_routes::delete_user,
                user_routes::put_user,
                post_routes::get_post_by_id,
                post_routes::search_posts,
                post_routes::create_post,
                post_routes::put_post,
                post_routes::delete_post,
                comment_routes::get_comments,
                comment_routes::delete_comment,
                comment_routes::put_comment,
                comment_routes::post_comment,
                comment_like_routes::comment_is_liked_by_user,
                comment_like_routes::like_comment,
                comment_like_routes::unlike_comment,
                post_like_routes::post_is_liked_by_user,
                post_like_routes::like_post,
                post_like_routes::unlike_post
            ],
        )
}
