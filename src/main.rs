pub mod db;
pub mod routes;
pub mod schema;
use crate::routes::cors::cors_fairing;
use routes::{auth_routes, comment_routes, index::index, like_routes, post_routes, user_routes};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let database_url = db::db_connection::DbPoolManager::get_database_url_from_dotenv();
    let pool = db::db_connection::DbPoolManager::init_pool(database_url.as_str());
    rocket::build().manage(pool).attach(cors_fairing()).mount(
        "/api/v1",
        routes![
            index,
            auth_routes::user_new,
            auth_routes::login,
            user_routes::get_user,
            user_routes::user_all,
            user_routes::delete_user,
            user_routes::put_user,
            post_routes::get_post,
            post_routes::get_all_posts,
            post_routes::post_post,
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
