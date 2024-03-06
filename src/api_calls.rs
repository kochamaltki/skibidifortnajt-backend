use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use sqlite::{Connection, State, Value};
use warp::reply::Json;
use warp::Filter;
use std::time::SystemTime;
use crate::get_token::get_token;
use crate::verify_token::{self, Claims};
use crate::check_banned::check_banned;
use crate::purge_data::purge_data;



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
    pub post_list: Vec<String>
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

fn check_tag(connection: &Connection, tag: &String) -> bool {
    let query = "SELECT tag_id FROM tags WHERE tag_name = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, tag.as_str())).unwrap();

    if let Ok(State::Row) = statement.next() {
        true
    } else {
        false
    }
}

fn check_user_id(connection: &Connection, id: i64) -> bool {
    let query = "SELECT user_id FROM users WHERE user_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, id)).unwrap();

    if let Ok(State::Row) = statement.next() {
        true
    } else {
        false
    }
}

fn check_user_name(connection: &Connection, name: &String) -> bool {
    let query = "SELECT user_id FROM users WHERE user_name = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, name.as_str())).unwrap();

    if let Ok(State::Row) = statement.next() {
        true
    } else {
        false
    }
}

fn count_posts(connection: &Connection) -> Result<i64, &str> {
    let query = "SELECT COUNT(post_id) FROM posts";
    let mut statement = connection.prepare(query).unwrap();

    if let Ok(State::Row) = statement.next() {
        Ok(statement.read::<i64, _>(0).unwrap())
    } else {
        Err("Failed to count posts")
    }
}

fn count_users(connection: &Connection) -> Result<i64, &str> {
    let query = "SELECT COUNT(user_id) FROM users";
    let mut statement = connection.prepare(query).unwrap();

    if let Ok(State::Row) = statement.next() {
        Ok(statement.read::<i64, _>(0).unwrap())
    } else {
        Err("Failed to count users")
    }
}

fn add_post_db(connection: &Connection, post: Post) {
    let time_since_epoch: i64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    
    let query = "INSERT INTO posts VALUES (:post_id, :user_id, :date, :body)";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind::<&[(_, Value)]>(&[
                   (":post_id", post.post_id.into()), 
                   (":user_id", post.user_id.into()), 
                   (":date", time_since_epoch.into()), 
                   (":body", post.body.into())
    ][..]).unwrap();
    statement.next().unwrap();

    println!(
        "Added post with id {} for user id {} time since epoch {}", 
        post.post_id, 
        post.user_id,
        time_since_epoch
    );
}

fn add_user_db(connection: &Connection, request: &SignupRequest) -> Json {
    let user_count = count_users(connection).unwrap();
    let signup_query = "INSERT INTO users VALUES (:user_id, :user_name, :passwd)";
    let mut signup_statement = connection.prepare(signup_query).unwrap();
    signup_statement.bind::<&[(_, Value)]>(&[
                                           (":user_id", user_count.into()), 
                                           (":user_name", request.user_name.clone().into()), 
                                           (":passwd", request.passwd.clone().into()), 
    ][..]).unwrap();
    signup_statement.next().unwrap();

    println!("User {} created with id {}", request.user_name, user_count);
    warp::reply::json(&get_token(user_count))
}

fn get_id_passwd_adm(connection: &Connection, user: &String) -> Result<(i64, String, i64), String> {
    let query = "SELECT passwd, user_id, is_admin FROM users WHERE user_name = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user.as_str())).unwrap();

    if let Ok(State::Row) = statement.next() {
        Ok((statement.read::<i64, _>(1).unwrap(), statement.read::<String, _>(0).unwrap(), statement.read::<i64, _>(2).unwrap()))
    } else {
        Err("User does not exist".to_string())
    }
}

pub async fn get_posts_by_user(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT * FROM posts WHERE user_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_id)).unwrap();

    let mut post_list: Vec<Post> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        post_list.push(Post { 
            post_id: statement.read::<i64, _>("post_id").unwrap(),
            user_id: statement.read::<i64, _>("user_id").unwrap(), 
            date: statement.read::<i64, _>("date").unwrap(),
            body: statement.read::<String, _>("body").unwrap()
        });
    }

    let post = PostList {
        post_list
    };
    Ok(warp::reply::json(&post))
}

pub async fn get_post_by_id(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT * FROM posts WHERE post_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, post_id)).unwrap();

    if let Ok(State::Row) = statement.next() {
        let post = Post { 
            post_id: statement.read::<i64, _>("post_id").unwrap(),
            user_id: statement.read::<i64, _>("user_id").unwrap(), 
            date: statement.read::<i64, _>("date").unwrap(),
            body: statement.read::<String, _>("body").unwrap()
        };
        Ok(warp::reply::json(&post))
    } else {
        let post = Post { 
            post_id: -1,
            user_id: -1,
            date: -1,
            body: "".to_string()
        };
        Ok(warp::reply::json(&post))
    }
}

pub async fn get_posts() -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT * FROM posts";
    let mut statement = connection.prepare(query).unwrap();

    let mut post_list: Vec<Post> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        post_list.push(Post { 
            post_id: statement.read::<i64, _>("post_id").unwrap(),
            user_id: statement.read::<i64, _>("user_id").unwrap(), 
            date: statement.read::<i64, _>("date").unwrap(),
            body: statement.read::<String, _>("body").unwrap()
        });
    }

    let post = PostList {
        post_list
    };
    Ok(warp::reply::json(&post))
}

pub async fn get_user_name(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT user_name FROM users WHERE user_id = ?1";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_id)).unwrap();

    let name = if let Ok(State::Row) = statement.next() {
        statement.read::<String, _>(0).unwrap()
    } else {
        "".to_string()
    };
    Ok(warp::reply::json(&name))
}

pub async fn get_user_id(user_name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let query = "SELECT user_id FROM users WHERE user_name = ?";  
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_name.as_str())).unwrap();

    let id = if let Ok(State::Row) = statement.next() {
        statement.read::<i64, _>(0).unwrap()
    } else {
        -1
    };
    Ok(warp::reply::json(&id))
}	

pub async fn post_post(request: PostCreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
		Ok(val) => { token = val }
        Err(_) => {
			return Ok(
                warp::reply::with_status(
            	    format!("Wrong token"),
                	warp::http::StatusCode::UNAUTHORIZED,
        	    )
            );
		}
	}

    if check_banned(token.claims.uid) == 1 {
        println!("User {} not allowed to post", token.claims.uid);
        return Ok(warp::reply::with_status(
            format!("Not allowed to post!"),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    };


    let id = token.claims.uid;
    
    let connection = sqlite::open("projekt-db").unwrap();
    
    if !check_user_id(&connection, id) {
        return Ok(
            warp::reply::with_status(
                format!("There is no user with id {}\n", id),
                warp::http::StatusCode::NOT_FOUND,
            )
        );
    };
    
    let post_count = count_posts(&connection).unwrap();
    
    add_post_db(
        &connection, 
        Post { 
            post_id: post_count,
            user_id: id, 
            date: -1,
            body: request.body
        }
    );

    Ok(
        warp::reply::with_status (
            format!("Post added for user with id {}\n", id),
            warp::http::StatusCode::CREATED,
        )
    )
}



pub async fn login(request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let name = request.user_name;

    match get_id_passwd(&connection, &name) {
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
    let connection = sqlite::open("projekt-db").unwrap();

    if check_user_name(&connection, &request.user_name) {
        let r = "User already exists!".to_string();
        Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::CONFLICT,
        ))

    } else {

        let token = add_user_db(&connection, &request);

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
            return Ok(warp::reply::with_status(
                    format!("Wrong token"),
                    warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }
    let connection = sqlite::open("projekt-db").unwrap();
    let id = token.claims.uid;
    if check_user_id(&connection, id) {
        let delete_query = "DELETE FROM users WHERE user_id = ?";
        let mut delete_statement = connection.prepare(delete_query).unwrap();
        delete_statement.bind((1, id)).unwrap();
        delete_statement.next().unwrap();

        drop(statement);
        drop(delete_statement); // close the previous connection
        drop(connection);

        purge_data(id);

        println!("User deletet with id : {}", id);
        Ok(warp::reply::with_status(
                format!("Delete succesful!"),
                warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
                format!("User does not exist!"),
                warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn upgrade_user(request: UserUpgradeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => {token = val}
        Err(_) => {
            return Ok(warp::reply::with_status(
                    format!("Wrong token"),
                    warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    if token.claims.is_admin != 1 {
        return Ok(warp::reply::with_status(
                format!("Not admin"),
                warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = sqlite::open("projekt-db").unwrap();
    let id = request.user_id;
    let query = "SELECT user_name FROM users WHERE user_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, id)).unwrap();

    if let Ok(State::Row) = statement.next() {
        let upgrade_query = "UPDATE users SET is_admin=1 WHERE user_id = ?";
        let mut upgrade_statement = connection.prepare(upgrade_query).unwrap();
        upgrade_statement.bind((1, id)).unwrap();
        upgrade_statement.next().unwrap();
        println!("User upgraded with id: {}", request.user_id);
        Ok(warp::reply::with_status(
                format!("Upgrade succesful!"),
                warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
                format!("User does not exist!"),
                warp::http::StatusCode::NOT_FOUND,
        ))
    }
}


pub async fn ban_user(request: UserBanRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => {token = val}
        Err(_) => {
            return Ok(warp::reply::with_status(
                    format!("Wrong token"),
                    warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    if token.claims.is_admin != 1 {
        return Ok(warp::reply::with_status(
                format!("Not admin"),
                warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = sqlite::open("projekt-db").unwrap();
    let id = request.user_id;
    let query = "SELECT user_name FROM users WHERE user_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, id)).unwrap();

    if let Ok(State::Row) = statement.next() {
        let upgrade_query = "UPDATE users SET is_banned=1 WHERE user_id = ?";
        let mut upgrade_statement = connection.prepare(upgrade_query).unwrap();
        upgrade_statement.bind((1, id)).unwrap();
        upgrade_statement.next().unwrap();

        drop(upgrade_statement);
        drop(statement); // close the previous connection
        drop(connection);

        purge_data(id);

        println!("User banned with id: {}", request.user_id);
        Ok(warp::reply::with_status(
                format!("Ban succesfull!"),
                warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
                format!("User does not exist!"),
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

