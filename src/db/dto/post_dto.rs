use serde::Serialize;
use uuid::Uuid;

use crate::db::models::{post_models::Post, user_models::User};

#[derive(Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub header: String,
    pub text: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub edit_date: chrono::DateTime<chrono::Utc>,
    pub author_username: String,
    pub author_name: Option<String>,
}

impl PostResponse {
    pub fn from_post(p: Post, u: User) -> Self {
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
            author_name: author_name,
        }
    }
}
