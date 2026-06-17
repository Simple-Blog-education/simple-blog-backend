use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use thiserror::Error;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    alg: String,
    typ: String,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Auth = 1,
    Refresh = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    user_id: Uuid,
    auth: TokenType,
}

impl Claims {
    pub fn new(username: String, user_id: Uuid, auth: TokenType) -> Claims {
        Claims {
            username,
            user_id,
            auth,
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}

pub fn get_default_secret() -> String {
    let _ = dotenv();
    return env::var("JWT_SECRET").expect("JWT Secret missing!");
}

pub struct JWT;

impl JWT {
    pub fn verify_signature(secret: &str, token: &str) -> Result<Claims, JWTError> {
        let parts: Vec<&str> = token.split(".").collect();
        if parts.len() != 3 {
            return Err(JWTError::BadFormat);
        }

        let (header_base64, payload_base64, signature_base64) = (parts[0], parts[1], parts[2]);

        let signature_bytes = URL_SAFE_NO_PAD
            .decode(signature_base64)
            .map_err(|_| JWTError::Invalid)?;

        let data_to_hash = format!("{}.{}", header_base64, payload_base64);
        let mut mac =
            HmacSha256::new_from_slice(&secret.as_bytes()).map_err(|_| JWTError::Invalid)?;
        mac.update(data_to_hash.as_bytes());
        mac.verify_slice(&signature_bytes)
            .map_err(|_| JWTError::Invalid)?;

        let payload_json = URL_SAFE_NO_PAD
            .decode(payload_base64)
            .map_err(|_| JWTError::Invalid)?;

        let claims: Claims =
            serde_json::from_slice(&payload_json).map_err(|_| JWTError::Invalid)?;

        Ok(claims)
    }

    pub fn make_token(claims: &Claims, secret: String) -> Result<String, JWTError> {
        let header = Header::default();
        let header_json = serde_json::to_string(&header).map_err(|_| JWTError::Invalid)?;
        let header_base64 = URL_SAFE_NO_PAD.encode(header_json);

        let payload_json = serde_json::to_string(claims).map_err(|_| JWTError::Invalid)?;
        let payload_base64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

        let data_to_hash = format!("{}.{}", header_base64, payload_base64);

        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| JWTError::Invalid)?;
        mac.update(data_to_hash.as_bytes());

        let signature = mac.finalize().into_bytes();
        let signature_base64 = URL_SAFE_NO_PAD.encode(signature);

        Ok(format!(
            "{}.{}.{}",
            header_base64, payload_base64, signature_base64
        ))
    }
    pub fn get_payload() {
        todo!("Необходимо получить здесь имя пользователя из токена")
    }
}

#[derive(Debug, Error)]
pub enum JWTError {
    #[error("JWT token missing")]
    Missing,
    #[error("JWT token invalid")]
    Invalid,
    #[error("JWT bad format")]
    BadFormat,
}

pub struct Auth(pub String, pub Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = JWTError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                header.trim_start_matches("Bearer ").trim()
            }
            Some(_) => return Outcome::Error((Status::BadRequest, JWTError::BadFormat)),
            None => return Outcome::Error((Status::BadRequest, JWTError::Missing)),
        };

        let secret = get_default_secret();
        match JWT::verify_signature(&secret, token) {
            Ok(claims) => Outcome::Success(Auth(claims.username, claims.user_id)),
            Err(e) => Outcome::Error((Status::Unauthorized, e)),
        }
    }
}

// TODO: Проверка на текущую роль будет сделана в AuthService
