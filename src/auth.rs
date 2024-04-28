use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
// argon2 jest wolny generalnie ale nie az tak jak jest teraz, zmiana na release build powinna przyspieszyc
// https://www.reddit.com/r/rust/comments/1ajkqd7/argon2_slow_is_hashing_password/
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors, TokenData};
use crate::get_secret::get_secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub uid: i64,
   pub exp: u64,
   pub is_admin: i64,
}

pub fn verify_token(token: String) -> Result<TokenData<Claims>, errors::Error>{
   let file_contents = get_secret();
   let jwt_secret = file_contents.as_str().trim();
   // Claims is a struct that implements Deserialize
   decode::<Claims>(&token, &DecodingKey::from_base64_secret(jwt_secret).expect("Nie udalo sie zdekodowac sekretu"), &Validation::new(Algorithm::HS256))
}

pub fn get_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_hash(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}