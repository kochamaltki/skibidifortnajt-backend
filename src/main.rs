use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, reply::{self, Reply}, Filter};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub user_id: u32,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Messages {
    pub messages: Vec<Message>
}

pub struct User {
    pub id: u32,
    pub name: String
}

pub async fn get_message_by_user(user_id: u32, messages: Arc<RwLock<Vec<Message>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Messages {
        messages: messages.read().unwrap().iter().filter(|x| x.user_id == user_id).map(|x| x.clone()).collect()
    };
    Ok(warp::reply::json(&post))
}

pub async fn get_messages(messages: Arc<RwLock<Vec<Message>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Messages {
        messages: messages.write().unwrap().to_vec()
    };
    Ok(warp::reply::json(&post))
}

pub async fn post_message(message: Message, messages: Arc<RwLock<Vec<Message>>>) -> Result<impl warp::Reply, warp::Rejection> {
    messages.write().unwrap().push(message);

    Ok(warp::reply::with_status(
            "Added items to the grocery list",
            warp::http::StatusCode::CREATED,
    ))
}

fn post_json() -> impl Filter<Extract = (Message,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let messages: Arc<RwLock<Vec<Message>>> = Arc::new(RwLock::new(Vec::new()));
    let messages_filter = warp::any().map(move || messages.clone());

    let get_messages_by_user = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-user"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(messages_filter.clone())
        .and_then(get_message_by_user);

    let get_messages = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("all"))
        .and(warp::path::end())
        .and(messages_filter.clone())
        .and_then(get_messages);

    let post_message = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path::end())
        .and(post_json())
        .and(messages_filter.clone())
        .and_then(post_message);

    get_messages_by_user.or(post_message).or(get_messages)
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin();
    let routes = routes().with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await
}
