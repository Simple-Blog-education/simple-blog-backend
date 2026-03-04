use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

pub fn cors_fairing() -> Cors {
    let allowed_origins = AllowedOrigins::All;
    let allowed_methods = vec![
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete,
        Method::Options,
    ]
    .into_iter()
    .map(From::from)
    .collect();
    let allowed_headers = AllowedHeaders::All;
    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while creating CORS fairing")
}
