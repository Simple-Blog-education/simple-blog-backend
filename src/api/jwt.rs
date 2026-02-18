use base64::{Engine, engine::general_purpose, prelude::BASE64_URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    alg: String,
    typ: String
}

impl Header {
    pub fn new(alg: String, typ: String) -> Header {
        Header {
            alg, typ
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    username:String,
    role: String
}

impl Payload {
    pub fn new(username: String, role: String) -> Payload {
        Payload { username, role }
    }
}


pub const DEFAULT_SECRET: &str = "mysecret";

pub struct JWT {
    pub header: Header,
    pub payload: Payload,
    pub secret: String,
}
impl JWT {
    pub fn new(header: Header, payload: Payload, secret: String) -> JWT {
        JWT {
            header, payload, secret
        }
    }
    fn verify_signature(&self, token: &str) -> Result<String, String> {
        let parts: Vec<&str> = token.split(".").collect();
        if parts.len() != 3 {
            return Err("Invalid JWT: expected 3 parts".to_owned())
        }
        
        let (header_base64, payload_base64, signature_base64) = (parts[0], parts[1], parts[2]);

        let data_to_hash = format!("{}.{}", header_base64, payload_base64);

        let signature_bytes = BASE64_URL_SAFE_NO_PAD.decode(signature_base64).unwrap();

        let mut mac = HmacSha256::new_from_slice(&self.secret.as_bytes()).unwrap();

        mac.update(data_to_hash.as_bytes());

        let _ = mac.verify_slice(&signature_bytes);

        Ok("Success".to_string())
    }   
    pub fn make_token(&self) -> Result<String, String> {
        let header_json = serde_json::to_string(&self.header).unwrap();
        let header_base64 = general_purpose::STANDARD.encode(header_json);

        let payload_json = serde_json::to_string(&self.payload).unwrap();
        let payload_base64 = general_purpose::STANDARD.encode(payload_json);

        let data_to_hash = format!("{}.{}", header_base64, payload_base64);

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(data_to_hash.as_bytes());
        let signature = mac.finalize().into_bytes();

        let signature_base64 = BASE64_URL_SAFE_NO_PAD.encode(signature);
        Ok(format!("{}.{}.{}", header_base64, payload_base64, signature_base64))
    }
}
