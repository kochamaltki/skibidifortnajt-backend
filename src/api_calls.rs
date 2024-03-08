use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use tokio_rusqlite::{params, Connection};
use warp::reply::Json;
use warp::Filter;
use std::time::SystemTime;
use crate::get_token::get_token;
use crate::verify_token::{self, Claims};

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

async fn check_user_id(connection: &Connection, id: i64) -> bool {
    let query = "SELECT user_id FROM users WHERE user_id = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([id]).unwrap();
        if let Some(_) = rows.next().unwrap() {
            Ok(true)
        } else {
            Ok(false)
        }
    }).await.unwrap()
}

async fn check_user_name(connection: &Connection, name: String) -> bool {
    let query = "SELECT user_id FROM users WHERE user_name = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([name]).unwrap();
        if let Some(_) = rows.next().unwrap() {
            Ok(true)
        } else {
            Ok(false)
        }
    }).await.unwrap()
}

async fn check_banned(connection: &Connection, user_id: i64) -> bool {
    let query = "SELECT is_banned FROM users WHERE user_id = ?";

    if !check_user_id(connection, user_id).await {
        return true;
    }

    let is_banned = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap();
        if let Ok(row) = rows.next() {
            let row = row.unwrap();
            Ok(row.get::<_, i64>(0).unwrap() != 0)
        } else {
            Ok(true)
        }
    }).await.unwrap();

    is_banned
}

// async fn purge_data(connection: &Connection, user_id: i64) {
//     let query = "DELETE FROM posts WHERE user_id = ?";
//     connection.call(move |conn| {
//         let mut statement = conn.prepare(query).unwrap();
//         statement.execute(params![user_id]).unwrap();
//         Ok(0)
//     }).await.unwrap();
// }

async fn count_posts(connection: &Connection) -> Result<i64, &str> {
    let query = "SELECT COUNT(post_id) FROM posts";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            Ok(Ok(val.get(0).unwrap()))
        } else {
            Ok(Err("Failed to count users"))
        }
    }).await.unwrap()
}

async fn count_users(connection: &Connection) -> Result<i64, &str> {
    let query = "SELECT COUNT(user_id) FROM users";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            Ok(Ok(val.get(0).unwrap()))
        } else {
            Ok(Err("Failed to count users"))
        }
    }).await.unwrap()
}

async fn count_tags(connection: &Connection) -> Result<i64, &str> {
    let query = "SELECT COUNT(tag_id) FROM tags";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            Ok(Ok(val.get(0).unwrap()))
        } else {
            Ok(Err("Failed to count users"))
        }
    }).await.unwrap()
}

// async fn get_tag_by_id(connection: &Connection, id: i64) -> Result<String, i8> {
//     let query = "SELECT tag_name FROM tags WHERE tag_id = ?";
//     connection.call(move |conn| {
//         let mut statement = conn.prepare(query).unwrap();
//         let mut rows = statement.query([id]).unwrap();
//         if let Some(val) = rows.next().unwrap() {
//             Ok(Ok(val.get(0).unwrap()))
//         } else {
//             Ok(Err(-1))
//         }
//     }).await.unwrap()
// }

async fn get_tag_by_name(connection: &Connection, name: String) -> Result<i64, i8> {
    let query = "SELECT tag_id FROM tags WHERE tag_name = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([name]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            Ok(Ok(val.get(0).unwrap()))
        } else {
            Ok(Err(-1))
        }
    }).await.unwrap()
}

async fn add_post_tag_db(connection: &Connection, post_id: i64, tag_id: i64) {
    let query = "INSERT INTO posts_tags VALUES (?, ?)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute([post_id, tag_id]).unwrap();
        Ok(0)
    }).await.unwrap();
}

async fn add_tag_db(connection: &Connection, name: String) -> i64 {
    let tag_count = count_tags(connection).await.unwrap(); 
    let query = "INSERT INTO tags VALUES (:tag_id, :tag_name)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![tag_count, name]).unwrap();
        Ok(0)
    }).await.unwrap();
    tag_count
}

async fn add_post_db(connection: &Connection, post: Post, tags: Vec<String>) {
    let time_since_epoch: i64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;

    let query = "INSERT INTO posts VALUES (?, ?, ?, ?)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![post.post_id, post.user_id, time_since_epoch, post.body]).unwrap();
        Ok(0)
    }).await.unwrap();

    for tag in tags.iter() {
        match get_tag_by_name(connection, tag.clone()).await {
            Ok(id) => {
                add_post_tag_db(connection, post.post_id, id).await;
            },
            Err(_) => {
                let id = add_tag_db(connection, tag.clone()).await; 
                add_post_tag_db(connection, post.post_id, id).await;
            }
        }
    }

    println!(
        "Added post {} for user {}", 
        post.post_id, 
        post.user_id,
    );
}

async fn add_user_db(connection: &Connection, request: SignupRequest) -> Json {
    let user_count = count_users(connection).await.unwrap();
    let user_name = request.user_name.clone();
    
    let signup_query = "INSERT INTO users VALUES (:user_id, :user_name, :passwd, 0, 0)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(signup_query).unwrap();
        statement.execute(params![user_count, request.user_name, request.passwd]).unwrap();
        Ok(0)
    }).await.unwrap();

    println!("User {} created with id {}", user_name, user_count);
    warp::reply::json(&get_token(user_count, 0))
}

async fn get_id_passwd_adm(connection: &Connection, user: String) -> Result<(i64, String, i64), String> {
    let query = "SELECT passwd, user_id, is_admin FROM users WHERE user_name = ?";

    let ret = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query_map(params![user], |row| { Ok((row.get(1).unwrap(), row.get(0).unwrap(), row.get(2).unwrap())) }).unwrap();
        if let Some(row) = rows.next() {
            Ok(row.unwrap())
        } else {
            Ok((-1, "".to_string(), -1))
        }
    }).await;
    match ret {
        Ok(val) => {
            if val.0 == -1 {
                Err("User not found".to_string())
            } else {
                Ok(val)
            }
        },
        Err(_) => Err("Database error".to_string())
    }
    
}

pub async fn get_posts_by_user(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "SELECT * FROM posts WHERE user_id = ?";

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND
        ));
    }

    let post_list = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap(); 
        let mut post_vec: Vec<Post> = Vec::new();
        while let Ok(row) = rows.next() {
            let row = row.unwrap();
            post_vec.push(
                Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap()
                }
            );
        }
        Ok(post_vec)
    }).await.unwrap();

    let post = PostList {post_list};
    Ok(warp::reply::with_status(
        warp::reply::json(&post),
        warp::http::StatusCode::OK
    ))
}

pub async fn get_posts_by_tag(tag: String) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let tag_id = match get_tag_by_name(&connection, tag.clone()).await {
        Ok(val) => {
            val
        },
        Err(_) => {
            let r = "Tag not found";
            return Ok(warp::reply::with_status(
                    warp::reply::json(&r),
                    warp::http::StatusCode::NOT_FOUND
            ));
        }
    };

    let query = "
        SELECT posts.post_id, posts.user_id, posts.date, posts.body 
        FROM posts 
        JOIN posts_tags
        ON posts.post_id = posts_tags.post_id 
        WHERE posts_tags.tag_id = ?
    ";
    
    let post_list = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![tag_id]).unwrap(); 
        let mut post_vec: Vec<Post> = Vec::new();
        while let Ok(row) = rows.next() {
            let row = row.unwrap();
            post_vec.push(
                Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap()
                }
            );
        }
        Ok(post_vec)
    }).await.unwrap();

    let post = PostList {post_list};
    Ok(warp::reply::with_status(
        warp::reply::json(&post),
        warp::http::StatusCode::OK
    ))
}

pub async fn get_tags_from_post(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "
        SELECT tags.tag_name
        FROM posts_tags
        JOIN tags
        ON tags.tag_id=posts_tags.tag_id
        WHERE posts_tags.post_id = ?
    ";
    
    let tag_list = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![post_id]).unwrap(); 
        let mut tag_vec: Vec<String> = Vec::new();
        while let Ok(row) = rows.next() {
            let row = row.unwrap();
            tag_vec.push(
                row.get(0).unwrap()
            );
        }
        Ok(tag_vec)
    }).await.unwrap();

    let tags = TagList { tag_list };
    Ok(warp::reply::with_status(
        warp::reply::json(&tags),
        warp::http::StatusCode::OK
    ))
}

pub async fn get_posts() -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "SELECT * FROM posts";

    let post_list = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![]).unwrap(); 
        let mut post_vec: Vec<Post> = Vec::new();
        while let Ok(row) = rows.next() {
            let row = row.unwrap();
            post_vec.push(
                Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap()
                }
            );
        }
        Ok(post_vec)
    }).await.unwrap();

    let post = PostList {post_list};
    Ok(warp::reply::with_status(
        warp::reply::json(&post),
        warp::http::StatusCode::OK
    ))
}

pub async fn get_post_by_id(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "SELECT * FROM posts WHERE post_id = ?";

    let post = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![post_id]).unwrap(); 
        let post: Post;
        if let Ok(row) = rows.next() {
            let row = row.unwrap();
            post = Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap()
            };
        } else {
            post = Post {
                    post_id: -1,
                    user_id: -1,
                    date: -1,
                    body: "".to_string()
            };
        }
        Ok(post)
    }).await.unwrap();

    if post.post_id != -1 {
        Ok(warp::reply::with_status(
            warp::reply::json(&post),
            warp::http::StatusCode::OK
        ))
    } else {
        let r = "Post not found";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND
        ))
    }
}

pub async fn get_user_name(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "SELECT user_name FROM users WHERE user_id = ?";

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND
        ));
    }

    let name = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap();
        if let Ok(row) = rows.next() {
            let row =row.unwrap();
            Ok(row.get(0).unwrap())
        } else {
            Ok("".to_string())
        }
    }).await.unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&name),
        warp::http::StatusCode::OK
    ))
}

pub async fn get_user_id(user_name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "SELECT user_id FROM users WHERE user_name = ?";

    if !check_user_name(&connection, user_name.clone()).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND
        ));
    }

    let id = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_name]).unwrap();
        if let Ok(row) = rows.next() {
            let row =row.unwrap();
            Ok(row.get(0).unwrap())
        } else {
            Ok("".to_string())
        }
    }).await.unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&id),
        warp::http::StatusCode::OK
    ))
}	

pub async fn post_post(request: PostCreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
		Ok(val) => { token = val }
        Err(_) => {
            let r = "Wrong token";
			return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
        	));
		}
	}
    
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let id = token.claims.uid;

    if check_banned(&connection, token.claims.uid).await == true {
        println!("User {} not allowed to post", token.claims.uid);
        let r = "User is banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    };

    if !check_user_id(&connection, id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    };
    
    let post_count = count_posts(&connection).await.unwrap();
    
    add_post_db(
        &connection, 
        Post { 
            post_id: post_count,
            user_id: id, 
            date: -1,
            body: request.body
        },
        request.tags
    ).await;

    let r = "Post created";
    Ok(warp::reply::with_status (
        warp::reply::json(&r),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn login(request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let name = request.user_name;

    match get_id_passwd_adm(&connection, name.clone()).await {
        Ok((user_id, passwd, is_admin)) => {
            if passwd == request.passwd {
                println!("User {} logged in", name);
                Ok(warp::reply::with_status(
                    warp::reply::json(&get_token(user_id, is_admin)),
                    warp::http::StatusCode::OK,
                ))
            } else {
                println!("User {} failed to log in", name);
                let r="Password incorrect!".to_string();
                Ok(warp::reply::with_status(
                    warp::reply::json(&r),
                    warp::http::StatusCode::UNAUTHORIZED,
                ))
            }
        },
        Err(e) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&e),
                warp::http::StatusCode::NOT_FOUND,
            ))
        }
    }
}

pub async fn signup(request: SignupRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();

    if check_user_name(&connection, request.user_name.clone()).await {
        let r = "User already exists!".to_string();
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::CONFLICT,
        ))

    } else {
        let token = add_user_db(&connection, request).await;
        Ok(warp::reply::with_status(
            token, 
            warp::http::StatusCode::CREATED
        ))
    }
}

pub async fn delete_user(request: UserDeleteRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => {token = val}
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let id = token.claims.uid;
    if check_user_id(&connection, id).await {
        let delete_query = "DELETE FROM users WHERE user_id = ?";
        connection.call(move |conn| {
            let mut statement = conn.prepare(delete_query).unwrap();
            statement.execute(params![id]).unwrap();
            Ok(0)
        }).await.unwrap();

        //purge_data(&connection, id).await;

        println!("User {} deleted", id);
        let r = "User deleted";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::OK,
        ))
    } else {
        let r = "User not found";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn upgrade_user(request: UserUpgradeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => {token = val}
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    if token.claims.is_admin != 1 {
        let r = "User is not admin";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let id = request.user_id;

    if check_user_id(&connection, id).await {
        let update_query = "UPDATE users SET is_admin=1 WHERE user_id = ?";
        connection.call(move |conn| {
            let mut statement = conn.prepare(update_query).unwrap();
            statement.execute(params![id]).unwrap();
            Ok(0)
        }).await.unwrap();

        let r = "Upgrade succesful";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::OK,
        ))
    } else {
        let r = "User not found";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn ban_user(request: UserBanRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => {token = val}
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    if token.claims.is_admin != 1 {
        let r = "User is not admin";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let id = request.user_id;

    if check_user_id(&connection, id).await {
        let ban_query = "UPDATE users SET is_banned=1 WHERE user_id = ?";
        connection.call(move |conn| {
            let mut statement = conn.prepare(ban_query).unwrap();
            statement.execute(params![id]).unwrap();
            Ok(0)
        }).await.unwrap();

        //purge_data(&connection, id).await;

        println!("User banned with id: {}", request.user_id);
        let r = "Ban successful";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::OK,
        ))
    } else {
        let r = "User not found";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}


pub fn post_json() -> impl Filter<Extract = (PostCreateRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn login_json() -> impl Filter<Extract = (LoginRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn signup_json() -> impl Filter<Extract = (SignupRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn delete_json() -> impl Filter<Extract = (UserDeleteRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn upgrade_json() -> impl Filter<Extract = (UserUpgradeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn ban_json() -> impl Filter<Extract = (UserBanRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
