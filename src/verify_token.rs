use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors, TokenData};
use crate::get_secret::get_secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub uid: i64,
   pub role: String,
   pub exp: u64,
}

pub fn verify_token(token: String) -> Result<TokenData<Claims>, errors::Error>{
   let file_contents = get_secret();
   let jwt_secret = file_contents.as_str().trim();
   // Claims is a struct that implements Deserialize
   decode::<Claims>(&token, &DecodingKey::from_base64_secret(jwt_secret).expect("Nie udalo sie zdekodowac sekretu"), &Validation::new(Algorithm::HS256))
}
