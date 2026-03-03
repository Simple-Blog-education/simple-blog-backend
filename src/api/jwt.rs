use base64::{Engine, engine::general_purpose, prelude::BASE64_URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use rocket::{Request, http::Status, request::{FromRequest, Outcome}};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::api::jwt;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header<'a> {
    alg: &'a str,
    typ: &'a str
}

pub const DEFAULT_HEADER: jwt::Header = Header {
    alg: "HS256", 
    typ:"JWT"
};

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Auth = 1,
    Refresh = 2
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    username:String,
    role: String,
    auth: TokenType
}

impl Payload {
    pub fn new(username: String, role: String, auth: TokenType) -> Payload {
        Payload { username, role, auth }
    }
}


pub const DEFAULT_SECRET: &str = "mysecret";

pub struct JWT;

impl JWT {
    pub fn verify_signature(secret: String, token: &str) -> Result<String, String> {
        let parts: Vec<&str> = token.split(".").collect();
        if parts.len() != 3 {
            return Err("Invalid JWT: expected 3 parts".to_owned())
        }
        
        let (header_base64, payload_base64, signature_base64) = (parts[0], parts[1], parts[2]);

        let data_to_hash = format!("{}.{}", header_base64, payload_base64);

        let signature_bytes = BASE64_URL_SAFE_NO_PAD.decode(signature_base64).unwrap();

        let mut mac = HmacSha256::new_from_slice(&secret.as_bytes()).unwrap();

        mac.update(data_to_hash.as_bytes());

        let _ = mac.verify_slice(&signature_bytes);

        Ok("Success".to_string())
    }   
    pub fn make_token(header: Header, payload: Payload, secret: String) -> Result<String, String> {
        let header_json = serde_json::to_string(&header).unwrap();
        let header_base64 = general_purpose::STANDARD.encode(header_json);

        let payload_json = serde_json::to_string(&payload).unwrap();
        let payload_base64 = general_purpose::STANDARD.encode(payload_json);

        let data_to_hash = format!("{}.{}", header_base64, payload_base64);

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(data_to_hash.as_bytes());
        let signature = mac.finalize().into_bytes();

        let signature_base64 = BASE64_URL_SAFE_NO_PAD.encode(signature);
        Ok(format!("{}.{}.{}", header_base64, payload_base64, signature_base64))
    }
}

#[derive(Debug)]
pub enum JWTError {
    Missing = 1,
    Invalid = 2
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = JWTError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Error((Status::BadRequest, JWTError::Missing)),
            Some(key) if (JWT::verify_signature(jwt::DEFAULT_SECRET.to_owned(), key.split(" ").collect::<Vec<&str>>()[1]).unwrap() == "Success") => Outcome::Success(JWT),
            Some(_) => Outcome::Error((Status::BadRequest, JWTError::Invalid)),
        }
    }
}