use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
// argon2 jest wolny generalnie ale nie az tak jak jest teraz, zmiana na release build powinna przyspieszyc
// https://www.reddit.com/r/rust/comments/1ajkqd7/argon2_slow_is_hashing_password/
use jsonwebtoken::{decode, encode, errors, EncodingKey, Header, DecodingKey, Validation, Algorithm, TokenData};
use std::time::SystemTime; 
use std::fs;
use crate::types::Claims;

pub fn get_secret() -> String{
    let contents = fs::read_to_string("./SECRET")
        .expect("Should have been able to read the file");
    contents
}

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn get_token(user_id: i64, is_admin_value: i64) -> String {
    let file_contents = get_secret();
    let jwt_secret = file_contents.as_str().trim();
    let expiration = get_sys_time_in_secs() + 1209600; // wazny przez 10 dni

    let claims = Claims {
        uid: user_id,
        exp: expiration,
        is_admin: is_admin_value
    };
    let header = Header::new(Algorithm::HS256);
    let tkn = encode(&header, &claims, &EncodingKey::from_base64_secret(jwt_secret).expect("Nie udalo sie zdekodowac sekretu"));
    tkn.expect("REASON")
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