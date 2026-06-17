use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub mod comment_dto;
pub mod post_dto;
pub mod user_dto;
