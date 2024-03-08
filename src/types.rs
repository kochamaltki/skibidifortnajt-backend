use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i64,
    pub date: i64,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostList {
    pub post_list: Vec<Post>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TagList {
    pub tag_list: Vec<String>
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
