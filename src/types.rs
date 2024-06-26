use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub uid: i64,
   pub exp: u64,
   pub is_admin: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub post_id: i64,
    pub user_id: i64,
    pub date: i64,
    pub body: String,
    pub likes: i64,
    pub user_name: String,
    pub display_name: String,
    pub pfp_image: String,
    pub like_count: i64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub post_id: i64,
    pub comment_id: i64,
    pub user_id: i64,
    pub body: String,
    pub date: i64,
    pub user_name: String,
    pub display_name: String,
    pub pfp_image: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub user_id: i64,
    pub user_name: String,
    pub display_name: String,
    pub description: String,
    pub pfp_image: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostList {
    pub post_list: Vec<Post>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileList {
    pub profile_list: Vec<Profile>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TagList {
    pub tag_list: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommentList {
    pub comment_list: Vec<Comment>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageList {
    pub image_list: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LikeCount {
    pub like_count: i64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRequest {
    pub user_name: String,
    pub passwd: String,
    pub remember_password: bool
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignupRequest {
    pub user_name: String,
    pub passwd: String,
    pub remember_password: bool 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostCreateRequest {
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommentCreateRequest {
    pub post_id: i64,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LikeRequest {
    pub post_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnlikeRequest {
    pub post_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDeleteRequest {
    pub user_id: i64, // NOT NEEDED
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostDeleteRequest {
    pub post_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserUpgradeRequest {
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserBanRequest {
    pub user_id: i64,
    pub ban_length: i64,
    pub ban_message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserUnbanRequest {
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DisplayNameChangeRequest {
    pub new_display_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserNameChangeRequest {
    pub new_user_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DescriptionChangeRequest {
    pub new_description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddImageToPostRequest {
    pub image_id: i64,
    pub post_id: i64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetPFPRequest {
    pub image_id: i64,
    pub user_id: i64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemovePFPRequest {
    pub user_id: i64
}
