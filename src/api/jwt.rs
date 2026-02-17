struct JWTHeader {}

struct JWTPayload {}

trait CanBeSignedByJWT {
    fn gen_signature() {}
    fn verify_signature() {}
}

struct JWT {}

impl CanBeSignedByJWT for JWT {
    fn gen_signature() {}

    fn verify_signature() {}
}
