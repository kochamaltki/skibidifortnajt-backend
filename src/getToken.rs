use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;
use jsonwebtoken::errors::ErrorKind;

use crate::Serialize;
use crate::Deserialize;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"bajojajo";

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    uid: String,
    role: String,
    exp: usize,
}


pub fn create_jwt(user_id: i32) -> String {
    // let expiration = Utc::now()
    //     .checked_add_signed(chrono::Duration::seconds(60))
    //     .expect("valid timestamp")
    //     .timestamp();
    let claims = Claims {
        uid: user_id.to_string(),
        role: "user".to_string(),
        exp: 69696969 as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let tkn = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET));
    return tkn.expect("REASON");
}