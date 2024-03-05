use serde::{Deserialize, Serialize};
use sqlite::{State, Value};
use warp::Filter;
use std::time::SystemTime;
use crate::get_token::get_token;
use crate::verify_token;


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
    pub user_id: i64,
    pub body: String,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDeleteRequest {
    pub user_id: i64,
    pub token: String
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

pub async fn post_post(post: PostCreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
	match verify_token::verify_token(post.token) {
		Ok(val) => {
            if val.claims.uid != post.user_id {
                return Ok(warp::reply::with_status(
                        format!("Wrong token"),
                        warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
        }
		Err(_) => {
			return Ok(warp::reply::with_status(
            	    format!("Wrong token"),
                	warp::http::StatusCode::UNAUTHORIZED,
        	));
		}
	}
    let connection = sqlite::open("projekt-db").unwrap();
    let user_check_query = "SELECT user_id FROM users WHERE user_id = ?"; 
    let mut user_check_statement = connection.prepare(user_check_query).unwrap();
    user_check_statement.bind((1, post.user_id)).unwrap();

    if let Ok(State::Row) = user_check_statement.next() {
    } else {
        return Ok(warp::reply::with_status(
                format!("There is no user with id {}\n", post.user_id),
                warp::http::StatusCode::NOT_FOUND,
        ));
    };
    
    let count_query = "SELECT COUNT(post_id) FROM posts";
    let mut statement = connection.prepare(count_query).unwrap();

    let count = if let Ok(State::Row) = statement.next() {
        statement.read::<i64, _>(0).unwrap()
    } else {
        panic!("failed to get post count!");
    };

    let time_since_epoch: i64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let query = "INSERT INTO posts VALUES (:post_id, :user_id, :date, :body)";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind::<&[(_, Value)]>(&[
                   (":post_id", count.into()), 
                   (":user_id", post.user_id.into()), 
                   (":date", time_since_epoch.into()), 
                   (":body", post.body.into())
    ][..]).unwrap();
    statement.next().unwrap();
    
    println!("Added post with id {} for user id {} time since epoch {}", 
             count, 
             post.user_id,
             time_since_epoch);
    
    Ok(warp::reply::with_status(
            format!("Post added for user with id {}\n", post.user_id),
            warp::http::StatusCode::CREATED,
    ))
}



pub async fn login(request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    let name = request.user_name;

    let query = "SELECT passwd, user_id FROM users WHERE user_name = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, name.as_str())).unwrap();

    if let Ok(State::Row) = statement.next() {
        if statement.read::<String, _>(0).unwrap() == request.passwd{
            println!("User {} logged in", name);
            let user_id = statement.read::<i64, _>(0).unwrap();
            Ok(warp::reply::with_status(
                    warp::reply::json(&get_token(user_id)),
                    warp::http::StatusCode::OK,
            ))
        } else {
            println!("User {} failed to log in", name);
            let _r="Password incorrect!".to_string();
            Ok(warp::reply::with_status(
                    warp::reply::json(&_r),
                    warp::http::StatusCode::UNAUTHORIZED,
            ))
        }
    } else {
        let _r="User does not exist!".to_string();
        Ok(warp::reply::with_status(
                warp::reply::json(&_r),
                warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn signup(request: SignupRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = sqlite::open("projekt-db").unwrap();
    
    let count_query = "SELECT COUNT(user_id) FROM users";
    let mut count_statement = connection.prepare(count_query).unwrap();
    let count;
    if let Ok(State::Row) = count_statement.next() {
        count = count_statement.read::<i64, _>(0).unwrap()
    } else {
        panic!("failed to get user count!");
    };
    
    let name = request.user_name;
    let query = "SELECT user_name FROM users WHERE user_name = ?"; 
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, name.as_str())).unwrap();

    if let Ok(State::Row) = statement.next() {

        let _r = "User already exists!".to_string();
        Ok(warp::reply::with_status(
                warp::reply::json(&_r),
                warp::http::StatusCode::CONFLICT,
        ))

    } else {

        let signup_query = "INSERT INTO users VALUES (:user_id, :user_name, :passwd)";
        let mut signup_statement = connection.prepare(signup_query).unwrap();
        signup_statement.bind::<&[(_, Value)]>(&[
            (":user_id", count.into()), 
            (":user_name", name.clone().into()), 
            (":passwd", request.passwd.into()), 
        ][..]).unwrap();
        signup_statement.next().unwrap();

        println!("User {} created with id {}", name, count);
        let token = warp::reply::json(&get_token(count));
        
        Ok(warp::reply::with_status(
                token, 
                warp::http::StatusCode::CREATED
        ))
    }
}

pub async fn delete_user(request: UserDeleteRequest) -> Result<impl warp::Reply, warp::Rejection> {
    match verify_token::verify_token(request.token) {
        Ok(val) => {
            if val.claims.uid != request.user_id {
                return Ok(warp::reply::with_status(
                        format!("Wrong token"),
                        warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
        }
        Err(_) => {
            return Ok(warp::reply::with_status(
                    format!("Wrong token"),
                    warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }
    let connection = sqlite::open("projekt-db").unwrap();
    let id = request.user_id;
    let query = "SELECT passwd FROM users WHERE user_id = ?"; // sqli
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, id)).unwrap();

    if let Ok(State::Row) = statement.next() {
        let delete_query = "DELETE FROM users WHERE user_id = ?"; // sqli
        let mut delete_statement = connection.prepare(delete_query).unwrap();
        delete_statement.bind((1, id)).unwrap();

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

