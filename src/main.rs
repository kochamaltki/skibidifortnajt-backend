use warp::Filter;
mod get_token;
mod get_secret;
mod verify_token;
mod api_calls;
use crate::api_calls::*;


// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Post {
//     pub post_id: i64,
//     pub user_id: i64,
//     pub date: i64,
//     pub body: String,
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct PostList {
//     pub post_list: Vec<Post>
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct LoginRequest {
//     pub user_name: String,
//     pub passwd: String
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct SignupRequest {
//     pub user_name: String,
//     pub passwd: String
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct PostCreateRequest {
//     pub user_id: i64,
//     pub body: String
// }

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct UserDeleteRequest {
//     pub user_id: i64
// }

// pub struct User {
//     pub user_id: i64,
//     pub user_name: String,
//     pub passwd: String
// }



pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_posts_by_user = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-user"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_posts_by_user);

    let get_posts = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("all"))
        .and(warp::path::end())
        .and_then(get_posts);
    
    let get_user_name = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("user"))
        .and(warp::path("name"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_user_name);
    
    let get_user_id = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("user"))
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_user_id);

    let make_token = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("token"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(make_token);

    let post_post = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("add-post"))
        .and(warp::path::end())
        .and(post_json())
        .and_then(post_post);

    let login = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(login_json())
        .and_then(login);

    let signup = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("signup"))
        .and(warp::path::end())
        .and(signup_json())
        .and_then(signup);
    
    let delete = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("delete-user"))
        .and(warp::path::end())
        .and(delete_json())
        .and_then(delete_user);

    get_posts_by_user
        .or(post_post)
        .or(get_posts)
        .or(login)
        .or(signup)
        .or(get_user_name)
        .or(delete)
        .or(get_user_id)
        .or(make_token)
}

#[tokio::main]
async fn main() {
    let cors = warp::cors().allow_any_origin();
    let routes = routes().with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

}
