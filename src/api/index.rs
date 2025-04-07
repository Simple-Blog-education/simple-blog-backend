#[get("/")]
pub fn index() -> &'static str {
    "Health check!"
}

