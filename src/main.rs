use serde::{Deserialize, Serialize};
use sqlite::State;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub user_id: i32,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostList {
    pub post_list: Vec<Post>
}

pub struct User {
    pub id: i32,
    pub name: String
}

pub async fn get_message_by_user(user_id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT * FROM posts WHERE user_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_id as i64)).unwrap();

    let mut post_list: Vec<Post> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        post_list.push(Post { 
            user_id: statement.read::<i64, _>("user_id").unwrap() as i32, 
            body: statement.read::<String, _>("body").unwrap()
        });
    }

    let post = PostList {
        post_list
    };
    Ok(warp::reply::json(&post))
}

pub async fn get_messages() -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT * FROM posts";
    let mut statement = connection.prepare(query).unwrap();

    let mut post_list: Vec<Post> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        post_list.push(Post { 
            user_id: statement.read::<i64, _>("user_id").unwrap() as i32, 
            body: statement.read::<String, _>("body").unwrap()
        });
    }

    let post = PostList {
        post_list
    };
    Ok(warp::reply::json(&post))
}

pub async fn post_message(message: Post) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = format!("INSERT INTO posts VALUES ({}, '{}')", message.user_id, message.body);
    connection.execute(query).unwrap();
    
    Ok(warp::reply::with_status(
            format!("Post added for user with id: {}\n", message.user_id),
            warp::http::StatusCode::CREATED,
    ))
}

fn post_json() -> impl Filter<Extract = (Post,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_messages_by_user = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-user"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_message_by_user);

    let get_messages = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("all"))
        .and(warp::path::end())
        .and_then(get_messages);

    let post_message = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("add-message"))
        .and(warp::path::end())
        .and(post_json())
        .and_then(post_message);

    get_messages_by_user.or(post_message).or(get_messages)
}

#[tokio::main]
async fn main() {
    let cors = warp::cors().allow_any_origin();
    let routes = routes().with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
