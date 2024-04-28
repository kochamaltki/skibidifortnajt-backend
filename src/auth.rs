use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
// argon2 jest wolny generalnie ale nie az tak jak jest teraz, zmiana na release build powinna przyspieszyc
// https://www.reddit.com/r/rust/comments/1ajkqd7/argon2_slow_is_hashing_password/

pub fn get_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_hash(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}