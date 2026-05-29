#[get("/")]
pub async fn index() -> &'static str {
    "Health check!"
}
