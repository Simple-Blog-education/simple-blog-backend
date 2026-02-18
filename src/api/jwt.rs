use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    alg: &'static str,
    typ: &'static str
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    username:&'static str,
    role: &'static str
}

const SECRET: &str = "mysecret";

struct JWT {
    header: Header,
    payload: Payload,
    secret: String,
}
impl JWT {
    fn new(header: Header, payload: Payload, secret: String) -> JWT {
        JWT {
            header, payload, secret
        }
    }
    fn gen_signature() -> () {
        let data_to_hash = "";
    }
    fn verify_signature() {}
    fn make_token() {}
}
