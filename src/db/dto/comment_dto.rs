use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(FromForm)]
pub struct CommentListParams {
    pub post_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    #[field(default = 1)]
    pub page: i64,
    #[field(default = 20)]
    pub per_page: i64,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub username: String,
    pub text: String,
    pub likes: i64,
    pub create_date: DateTime<Utc>,
    pub is_liked: bool,
}
