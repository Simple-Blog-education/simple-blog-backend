use serde::Serialize;
use uuid::Uuid;

use crate::db::models::{post_models::Post, user_models::User};

#[derive(FromForm)]
pub struct PostSearchParams {
    pub query: Option<String>,
    #[field(default = 1)]
    pub page: i64,
    #[field(default = 10)]
    pub per_page: i64,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub header: String,
    pub text: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub edit_date: chrono::DateTime<chrono::Utc>,
    pub author_username: String,
    pub author_avatar: Option<String>,
    pub author_name: Option<String>,
    pub likes: i64,
    pub is_liked: bool,
}

impl PostResponse {
    pub fn from_post_and_user(p: Post, u: User, likes: i64, is_liked: bool) -> Self {
        let author_name = match (u.first_name.as_deref(), u.last_name.as_deref()) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(name), None) | (None, Some(name)) => Some(name.to_string()),
            (None, None) => None,
        };
        Self {
            id: p.id,
            header: p.header,
            text: p.text,
            create_date: p.create_date,
            edit_date: p.edit_date,
            author_username: u.username,
            author_avatar: u.avatar_url,
            author_name,
            likes,
            is_liked,
        }
    }
}
