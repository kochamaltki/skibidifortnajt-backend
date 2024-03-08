use jsonwebtoken::TokenData;
use tokio_rusqlite::params;
use tracing::info;
use warp::Filter;

use crate::get_token::get_token;
use crate::verify_token::{self, Claims};
use crate::types::*;
use crate::database_functions::*;

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

pub async fn post(request: PostCreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
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
        info!("User {} not allowed to post", token.claims.uid);
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

pub async fn react(request: ReactRequest) -> Result<impl warp::Reply, warp::Rejection> {
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
    let existed = add_reaction_db(&connection, token.claims.uid, request.post_id, request.reaction_type).await;

    if existed {
        let r = "Reaction already exists";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_ACCEPTABLE,
        ))
    } else {
        let r = "Reaction added";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::OK,
        ))
    }
}

pub async fn login(request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let name = request.user_name;

    match get_id_passwd_adm(&connection, name.clone()).await {
        Ok((user_id, passwd, is_admin)) => {
            if passwd == request.passwd {
                info!("User {} logged in", name);
                Ok(warp::reply::with_status(
                    warp::reply::json(&get_token(user_id, is_admin)),
                    warp::http::StatusCode::OK,
                ))
            } else {
                info!("User {} failed to log in", name);
                let r="Password incorrect".to_string();
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
        let r = "User already exists".to_string();
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

        purge_data(&connection, id).await;

        info!("User {} deleted", id);
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
            let r = "Wrong tokeyn";
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

        purge_data(&connection, id).await;

        info!("User banned with id: {}", request.user_id);
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

pub fn react_json() -> impl Filter<Extract = (ReactRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
