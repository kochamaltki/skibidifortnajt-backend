use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::SystemTime; 
use crate::get_secret::get_secret;



#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    uid: i64,
    role: String,
    exp: u64,
}

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}


pub fn get_token(user_id: i64) -> String {
    let file_contents = get_secret();
    let jwt_secret = file_contents.as_str();
    let expiration = get_sys_time_in_secs() + 864000; // wazny przez 10 dni

    let claims = Claims {
        uid: user_id,
        role: "user".to_string(),
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS512);
    let tkn = encode(&header, &claims, &EncodingKey::from_base64_secret(jwt_secret).expect("Nie udalo sie zdekodowac sekretu"));
    return tkn.expect("REASON");
}