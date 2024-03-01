use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio_postgres::{Client, Error, NoTls};
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
    pub id: u32,
    pub name: String
}

pub async fn get_message_by_user(user_id: i32, client: Arc<RwLock<Client>>) -> Result<impl warp::Reply, warp::Rejection> {
    let rows = client
        .write()
        .await
        .query("SELECT * FROM posts WHERE user_id = $1", &[&user_id])
        .await
        .unwrap();

    let post_list: Vec<Post> = rows
        .iter()
        .map(|x| {
            Post { user_id: x.get("user_id"), body: x.get("body") }
        })
        .collect();

    let post = PostList {
        post_list
    };
    Ok(warp::reply::json(&post))
}

pub async fn get_messages(client: Arc<RwLock<Client>>) -> Result<impl warp::Reply, warp::Rejection> {
    let rows = client
        .write()
        .await
        .query("SELECT * FROM posts", &[])
        .await
        .unwrap();

    let post_list: Vec<Post> = rows
        .iter()
        .map(|x| {
            Post { user_id: x.get("user_id"), body: x.get("body") }
        })
        .collect();

    let post = PostList {
        post_list
    };
    Ok(warp::reply::json(&post))
}

pub async fn post_message(message: Post, client: Arc<RwLock<Client>>) -> Result<impl warp::Reply, warp::Rejection> {
    client
        .write().await
        .query("INSERT INTO posts VALUES ($1, $2)", &[&message.user_id, &message.body]).await
        .unwrap();
    
    Ok(warp::reply::with_status(
            format!("Post added for user with id: {}\n", message.user_id),
            warp::http::StatusCode::CREATED,
    ))
}

fn post_json() -> impl Filter<Extract = (Post,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn routes(client: Arc<RwLock<Client>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let filters = warp::any().map(move || client.clone());

    let get_messages_by_user = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-user"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(filters.clone())
        .and_then(get_message_by_user);

    let get_messages = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("all"))
        .and(warp::path::end())
        .and(filters.clone())
        .and_then(get_messages);

    let post_message = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path::end())
        .and(post_json())
        .and(filters.clone())
        .and_then(post_message);

    get_messages_by_user.or(post_message).or(get_messages)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=projekt-db user=dr", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let cors = warp::cors().allow_any_origin();
    let routes = routes(Arc::new(RwLock::new(client))).with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
