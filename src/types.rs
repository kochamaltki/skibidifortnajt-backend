use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i64,
    pub date: i64,
    pub body: String,
}

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct User {
//     pub user_id: i64,
//     pub user_name: String,
//     pub display_name: String,
//     pub profile_picture: String,
//     pub description: String 
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostList {
    pub post_list: Vec<Post>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TagList {
    pub tag_list: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReactionCountMap {
    pub reaction_count_map: HashMap<i64, i64>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRequest {
    pub user_name: String,
    pub passwd: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignupRequest {
    pub user_name: String,
    pub passwd: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostCreateRequest {
    pub body: String,
    pub tags: Vec<String>,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReactRequest {
    pub post_id: i64,
    pub reaction_type: i64,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDeleteRequest {
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserUpgradeRequest {
    pub user_id: i64,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserBanRequest {
    pub user_id: i64,
    pub token: String
}
