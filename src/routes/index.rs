use bcrypt::{hash, DEFAULT_COST};

#[get("/")]
pub async fn index() -> &'static str {
    "Health check!"
}
