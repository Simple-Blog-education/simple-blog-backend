#[derive(FromForm)]
pub struct CommentListParams {
    #[field(default = 1)]
    pub page: i64,
    #[field(default = 20)]
    pub per_page: i64,
}
