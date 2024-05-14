use crate::database_functions::*;
use crate::types::*;
use crate::auth::*;
use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};

use tokio_rusqlite::params;
use tracing::{error, info};
use warp::filters::multipart::FormData;
use warp::reject::{Reject, Rejection};

use std::time::SystemTime;
use warp::Filter;

pub async fn get_posts_by_user(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT posts.*, users.user_name FROM posts JOIN users ON users.user_id=posts.user_id WHERE users.user_id = ?";

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if check_banned(&connection, user_id).await {
        let r = "This user has been banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let post_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![user_id]).unwrap();
            let mut post_vec: Vec<Post> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                post_vec.push(Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap(),
                    likes: row.get(4).unwrap(),
                    user_name: row.get(5).unwrap(),
                });
            }
            Ok(post_vec)
        })
        .await
        .unwrap();

    let post = PostList { post_list };
    Ok(warp::reply::with_status(
        warp::reply::json(&post),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_posts_by_tag(tag: String) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let tag_id = match get_tag_by_name(&connection, tag.clone()).await {
        Ok(val) => val,
        Err(_) => {
            let r = "Tag not found";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::NOT_FOUND,
            ));
        }
    };

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let query = format!(
        "
        SELECT posts.post_id, posts.user_id, posts.date, posts.body, users.user_id 
        FROM posts 
        JOIN posts_tags
        ON posts.post_id = posts_tags.post_id 
        JOIN users
        ON posts.user_id = users.user_id
        WHERE posts_tags.tag_id = ? 
        AND posts.user_id NOT IN 
        (SELECT user_id FROM bans WHERE is_active = 1 AND expires_on > {})
    ",
        timestamp
    );
    let post_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(&query).unwrap();
            let mut rows = statement.query(params![tag_id]).unwrap();
            let mut post_vec: Vec<Post> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                post_vec.push(Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap(),
                    likes: row.get(4).unwrap(),
                    user_name: row.get(5).unwrap(),
                });
            }
            Ok(post_vec)
        })
        .await
        .unwrap();

    let post = PostList { post_list };
    Ok(warp::reply::with_status(
        warp::reply::json(&post),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_posts() -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let query = format!(
        "
        SELECT posts.*, users.user_name       
        FROM posts
        JOIN users
        ON posts.user_id = users.user_id
        WHERE posts.user_id NOT IN
        (SELECT user_id FROM bans WHERE is_active = 1 AND expires_on > {})",
        timestamp
    );

    let post_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(&query).unwrap();
            let mut rows = statement.query(params![]).unwrap();
            let mut post_vec: Vec<Post> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                post_vec.push(Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap(),
                    likes: row.get(4).unwrap(),
                    user_name: row.get(5).unwrap(),
                });
            }
            Ok(post_vec)
        })
        .await
        .unwrap();

    let post = PostList { post_list };
    Ok(warp::reply::with_status(
        warp::reply::json(&post),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_comments_from_post(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
    let query = "
        SELECT comments.*, users.user_name
        FROM comments
        JOIN users
        ON users.user_id = comments.user_id
        WHERE comments.post_id = ?
    ";
    
    if !check_post(&connection, post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }
    
    let comment_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![post_id]).unwrap();
            let mut comment_vec: Vec<Comment> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                comment_vec.push(
                    Comment { 
                        post_id: row.get(0).unwrap(), 
                        comment_id: row.get(1).unwrap(), 
                        user_id: row.get(2).unwrap(), 
                        body: row.get(3).unwrap(), 
                        date: row.get(4).unwrap(), 
                        likes: row.get(5).unwrap(), 
                        user_name: row.get(6).unwrap() 
                    }
                );
            }
            Ok(comment_vec)
        })
        .await
        .unwrap();

    let tags = CommentList { comment_list };
    Ok(warp::reply::with_status(
        warp::reply::json(&tags),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_tags_from_post(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "
        SELECT tags.tag_name
        FROM posts_tags
        JOIN tags
        ON tags.tag_id=posts_tags.tag_id
        WHERE posts_tags.post_id = ?
    ";

    if !check_post(&connection, post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    let tag_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![post_id]).unwrap();
            let mut tag_vec: Vec<String> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                tag_vec.push(row.get(0).unwrap());
            }
            Ok(tag_vec)
        })
        .await
        .unwrap();

    let tags = TagList { tag_list };
    Ok(warp::reply::with_status(
        warp::reply::json(&tags),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_like_from_post_by_user(
    post_id: i64,
    user_id: i64,
) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "
        SELECT user_id FROM likes
        WHERE user_id=? AND post_id=?
    ";

    if !check_post(&connection, post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    let exists = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![user_id, post_id]).unwrap();
            if let Ok(Some(_)) = rows.next() {
                Ok(true)
            } else {
                Ok(false)
            }
        })
        .await
        .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&exists),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_post_by_id(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT posts.*, users.user_name FROM posts
        JOIN users
        ON posts.user_id = users.user_id
        WHERE posts.post_id = ?";

    let post = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![post_id]).unwrap();
            let post: Post;
            if let Ok(Some(row)) = rows.next() {
                post = Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap(),
                    likes: row.get(4).unwrap(),
                    user_name: row.get(5).unwrap(),
                };
            } else {
                post = Post {
                    post_id: -1,
                    user_id: 0,
                    date: 0,
                    body: "".to_string(),
                    likes: 0,
                    user_name: "".to_string(),
                };
            }
            Ok(post)
        })
        .await
        .unwrap();

    if check_banned(&connection, post.user_id).await == true {
        let r = "The user who made this post has been banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    if post.post_id != -1 {
        Ok(warp::reply::with_status(
            warp::reply::json(&post),
            warp::http::StatusCode::OK,
        ))
    } else {
        let r = "Post not found";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn get_profile_by_id(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    if check_banned(&connection, user_id).await == true {
        let r = "This user has been banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED, // nw czy nie ma lepszego kodu do zwrocenia
        ));
    }

    let query = "
        SELECT users.user_id, users.user_name, 
               users.display_name, users.description,
               images.image_file
        FROM users 
        JOIN images ON images.image_id=users.pfp_id
        WHERE users.user_id = ?
    ";

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    let profile = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![user_id]).unwrap();
            let profile: Profile;
            if let Ok(Some(row)) = rows.next() {
                let pfp = match row.get::<_, String>(4) {
                    Ok(val) => format!("pfp_{}", val),
                    Err(_) => "".to_string()
                };
                profile = Profile {
                    user_id: row.get(0).unwrap(),
                    user_name: row.get(1).unwrap(),
                    display_name: row.get(2).unwrap(),
                    description: row.get(3).unwrap(),
                    pfp_image: pfp
                };
            } else {
                profile = Profile {
                    user_id: -1,
                    user_name: "".to_string(),
                    display_name: "".to_string(),
                    description: "".to_string(),
                    pfp_image: "".to_string()
                };
            }
            Ok(profile)
        })
        .await
        .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&profile),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_user_name(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT user_name FROM users WHERE user_id = ?";

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if check_banned(&connection, user_id).await == true {
        let r = "This user has been banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let name = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![user_id]).unwrap();
            if let Ok(Some(row)) = rows.next() {
                Ok(row.get(0).unwrap())
            } else {
                Ok("".to_string())
            }
        })
        .await
        .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&name),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_user_id(user_name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT user_id FROM users WHERE user_name = ?";

    if !check_user_name(&connection, user_name.clone()).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    let id = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![user_name]).unwrap();
            if let Ok(Some(row)) = rows.next() {
                Ok(row.get(0).unwrap())
            } else {
                Ok(-1)
            }
        })
        .await
        .unwrap();

    Ok(warp::reply::with_status(
        warp::reply::json(&id),
        warp::http::StatusCode::OK,
    ))
}

pub async fn get_images_from_post(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT image_file 
                 FROM posts_images 
                 JOIN images ON images.image_id=posts_images.image_id 
                 WHERE post_id = ?";

    if !check_post(&connection, post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    let images = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![post_id]).unwrap();
            let mut image_ids: Vec<String> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                image_ids.push(row.get::<_, String>(0).unwrap());
            }
            Ok(image_ids)
        })
        .await
        .unwrap();

    let image_id_list = ImageList { image_list: images };
    Ok(warp::reply::with_status(
        warp::reply::json(&image_id_list),
        warp::http::StatusCode::OK,
    ))
}

pub async fn validate_token(token: Option<String>) -> Result<impl warp::Reply, warp::Rejection> {
    match token {
        Some(token) => match verify_token(token) {
            Ok(val) => {
                let r = val.claims.uid;
                Ok(warp::reply::with_status(
                    warp::reply::json(&r),
                    warp::http::StatusCode::OK,
                ))
            }
            Err(_) => {
                let r = "Wrong token";
                Ok(warp::reply::with_status(
                    warp::reply::json(&r),
                    warp::http::StatusCode::UNAUTHORIZED,
                ))
            }
        },
        None => {
            let r = "No token";
            Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ))
        }
    }
}

pub async fn post(
    token: String,
    request: PostCreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            info!("{}", r);
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if check_banned(&connection, token.claims.uid).await {
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

    add_upload_db(&connection, token.claims.uid, 5).await;
    let post_id = get_next_post_id(&connection).await.unwrap();

    add_post_db(
        &connection,
        Post {
            post_id,
            user_id: id,
            date: -1,
            body: request.body,
            likes: 0,
            user_name: "".to_string(),
        },
        request.tags,
    )
    .await;

    Ok(warp::reply::with_status(
        warp::reply::json(&post_id),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn comment(
    token: String,
    request: CommentCreateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            info!("{}", r);
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if !check_post(&connection, request.post_id).await {
        info!("Post {} not found", request.post_id);
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if check_banned(&connection, token.claims.uid).await {
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

    add_upload_db(&connection, token.claims.uid, 3).await;
    let comment_id = get_next_comment_id(&connection, request.post_id).await.unwrap();

    add_comment_db(
        &connection,
        request.post_id,
        comment_id,
        token.claims.uid,
        request.body
    )
    .await;

    Ok(warp::reply::with_status(
        warp::reply::json(&comment_id),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn react(
    token: String,
    request: LikeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if check_banned(&connection, token.claims.uid).await {
        info!("User {} not allowed to react", token.claims.uid);
        let r = "User is banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    };

    if !check_post(&connection, request.post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    add_upload_db(&connection, token.claims.uid, 1).await;
    let existed = add_like_db(&connection, token.claims.uid, request.post_id).await;

    if existed {
        let r = "Like already exists";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_ACCEPTABLE,
        ))
    } else {
        let r = "Like added";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::OK,
        ))
    }
}

pub async fn unreact(
    token: String,
    request: UnlikeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if check_banned(&connection, token.claims.uid).await {
        info!("User {} not allowed to react", token.claims.uid);
        let r = "User is banned";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    };

    if !check_post(&connection, request.post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    add_upload_db(&connection, token.claims.uid, 1).await;
    let existed = remove_like_db(&connection, token.claims.uid, request.post_id).await;

    if existed {
        let r = "Like doesn't exists";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_ACCEPTABLE,
        ))
    } else {
        let r = "Like removed";
        Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::OK,
        ))
    }
}

#[derive(Debug)]
struct IncorrectPassword;
impl Reject for IncorrectPassword {}

#[derive(Debug)]
struct UserBanned;
impl Reject for UserBanned {}

#[derive(Debug)]
struct UserNotFound;
impl Reject for UserNotFound {}

pub async fn login(request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let name = request.user_name;

    match get_id_passwd_adm(&connection, name.clone()).await {
        Ok((user_id, hash, is_admin)) => {
            if check_banned(&connection, user_id).await {
                info!("Can't log in user {}, reason - ban", user_id);
                return Err(warp::reject::custom(UserBanned));
            };

            if verify_hash(request.passwd, hash) {
                info!("User {} logged in", name);
                let token = get_token(user_id, is_admin);
                let mut cookie_params =
                    "Path=/; HttpOnly; Secure; SameSite=None; Partitioned;".to_string();
                if request.remember_password == true {
                    cookie_params += "Max-Age=1209600;";
                }
                Ok(warp::reply::with_header(
                    token.clone(),
                    "set-cookie",
                    format!("token={}; {}", token, cookie_params),
                ))
            } else {
                info!("User {} failed to log in", name);
                Err(warp::reject::custom(IncorrectPassword))
            }
        }
        Err(_) => Err(warp::reject::custom(UserNotFound)),
    }
}

pub async fn logout(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    match verify_token(token) {
        Ok(_) => {}
        Err(_) => {
            return Err(warp::reject::custom(WrongToken));
        }
    };

    let cookie_params =
        "Path=/; HttpOnly; Secure; SameSite=None; Partitioned; Max-Age=0".to_string();
    Ok(warp::reply::with_header(
        "Logout",
        "set-cookie",
        format!("token=\"\"; {}", cookie_params),
    ))
}

#[derive(Debug)]
struct UserAlereadyExists;
impl Reject for UserAlereadyExists {}

#[derive(Debug)]
struct WrongToken;
impl Reject for WrongToken {}

pub async fn signup(request: SignupRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    if check_user_name(&connection, request.user_name.clone()).await {
        Err(warp::reject::custom(UserAlereadyExists))
    } else {
        let mut cookie_params = "Path=/; HttpOnly; Secure; SameSite=None; Partitioned;".to_string();
        if request.remember_password == true {
            cookie_params += "Max-Age=1209600;";
        }
        let token = add_user_db(&connection, request).await;
        Ok(warp::reply::with_header(
            token.clone(),
            "set-cookie",
            format!("token={}; {}", token, cookie_params),
        ))
    }
}

pub async fn delete_user(
    token: String,
    _request: UserDeleteRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("{}", token);
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            return Err(warp::reject::custom(WrongToken));
        }
    };
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;
    if check_user_id(&connection, id).await {
        let delete_query = "DELETE FROM users WHERE user_id = ?";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(delete_query).unwrap();
                statement.execute(params![id]).unwrap();
                Ok(0)
            })
            .await
            .unwrap();

        purge_data(&connection, id).await;

        info!("User {} deleted", id);
        let r = "User deleted";
        let res = warp::reply::with_status(r, warp::http::StatusCode::OK);
        let res = warp::reply::with_header(res, "Access-Control-Allow-Origin", "*");
        Ok(res)
    } else {
        Err(warp::reject::custom(UserNotFound))
    }
}

pub async fn upgrade_user(
    token: String,
    request: UserUpgradeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    if token.claims.is_admin != 1 {
        let r = "User is not admin";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = request.user_id;

    if check_user_id(&connection, id).await {
        let update_query = "UPDATE users SET is_admin=1 WHERE user_id = ?";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(update_query).unwrap();
                statement.execute(params![id]).unwrap();
                Ok(0)
            })
            .await
            .unwrap();

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

pub async fn ban_user(
    token: String,
    request: UserBanRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    if token.claims.is_admin != 1 {
        let r = "User is not admin";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = request.user_id;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expiration = timestamp + request.ban_length;
    if check_user_id(&connection, id).await {
        let ban_query = "INSERT INTO bans VALUES (?, ?, ?, ?, ?)";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(ban_query).unwrap();
                statement
                    .execute(params![id, timestamp, expiration, request.ban_message, 1])
                    .unwrap();
                Ok(0)
            })
            .await
            .unwrap();

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

pub async fn unban_user(
    token: String,
    request: UserUnbanRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    if token.claims.is_admin != 1 {
        let r = "User is not admin";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = request.user_id;
    if check_user_id(&connection, id).await {
        let unban_query = "UPDATE bans SET is_active = 0 WHERE user_id = ? AND is_active = 1";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(unban_query).unwrap();
                statement.execute(params![id]).unwrap();
                Ok(0)
            })
            .await
            .unwrap();

        info!("User unbanned with id: {}", request.user_id);
        let r = "Unban successful";
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

pub async fn change_display_name(
    token: String,
    request: DisplayNameChangeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if check_user_id(&connection, id).await {
        let change_query = "UPDATE users SET display_name= ? WHERE user_id = ?";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(change_query).unwrap();
                statement
                    .execute(params![request.new_display_name, id])
                    .unwrap();
                Ok(0)
            })
            .await
            .unwrap();

        info!(
            "Display name changed for user with id: {}",
            token.claims.uid
        );
        add_upload_db(&connection, token.claims.uid, 1).await;
        let r = "Display name change successful";
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

pub async fn change_description(
    token: String,
    request: DescriptionChangeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if check_user_id(&connection, id).await {
        let change_query = "UPDATE users SET description= ? WHERE user_id = ?";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(change_query).unwrap();
                statement
                    .execute(params![request.new_description, id])
                    .unwrap();
                Ok(0)
            })
            .await
            .unwrap();

        info!("Description changed for user with id: {}", token.claims.uid);
        add_upload_db(&connection, token.claims.uid, 1).await;
        let r = "Description change successful";
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

pub async fn upload_image(
    token: String,
    form: FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    let mut parts = form.into_stream();
    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "image/png" => {
                        file_ending = "png";
                    }
                    _ => {
                        let r = "Invalid image format";
                        return Ok(warp::reply::with_status(
                            warp::reply::json(&r),
                            warp::http::StatusCode::BAD_REQUEST,
                        ));
                    }
                },
                None => {
                    let r = "File type error";
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&r),
                        warp::http::StatusCode::BAD_REQUEST,
                    ));
                }
            }
            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await;
            let value = match value {
                Ok(val) => val,
                Err(_) => {
                    let r = "File read error";
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&r),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                }
            };
            let image_uuid = uuid::Uuid::new_v4().to_string();
            let file_name = format!("./media/images/{}.{}", image_uuid, file_ending);
            let pfp_file_name = format!("./media/images/pfp_{}.{}", image_uuid, file_ending);

            match add_image_db(&connection, format!("{}.{}", image_uuid, file_ending)).await {
                Ok(val) => {
                    tokio::fs::write(&file_name, value.clone()).await.map_err(|e| {
                        error!("error writing file: {}", e);
                        warp::reject::reject()
                    })?;
                    tokio::fs::write(&pfp_file_name, value).await.map_err(|e| {
                        error!("error writing file: {}", e);
                        warp::reject::reject()
                    })?;
                    add_upload_db(&connection, token.claims.uid, 10).await;
                    info!("created file: {}", file_name);
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&val),
                        warp::http::StatusCode::OK,
                    ));
                }
                Err(val) => {
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&val),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                }
            }
        }
    }

    let r = "Invalid request";
    Ok(warp::reply::with_status(
        warp::reply::json(&r),
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

pub async fn set_pfp(
    token: String,
    request: SetPFPRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if !check_image(&connection, request.image_id).await {
        let r = "Image not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if !check_user_id(&connection, token.claims.uid).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    add_upload_db(&connection, token.claims.uid, 1).await;

    match assign_image_to_user(&connection, token.claims.uid, request.image_id).await {
        Ok(_) => {
            let r = "PFP updated";
            Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::OK,
            ))
        }
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&e),
            warp::http::StatusCode::BAD_REQUEST,
        )),
    }
}

pub async fn add_image_to_post(
    token: String,
    request: AddImageToPostRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    let token = match verify_token(token) {
        Ok(val) => val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    };

    if is_limited(&connection, token.claims.uid).await && token.claims.is_admin == 0 {
        let r = "Ur too fast";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if !check_image(&connection, request.image_id).await {
        let r = "Image not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if !check_post(&connection, request.post_id).await {
        let r = "Post not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if token.claims.uid != get_user_from_post(&connection, request.post_id).await
        && token.claims.is_admin == 0
    {
        let r = "User not authorized";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    add_upload_db(&connection, token.claims.uid, 1).await;

    match assign_image_to_post_db(&connection, request.post_id, request.image_id).await {
        Ok(_) => {
            let r = "Image added to post";
            Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::OK,
            ))
        }
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&e),
            warp::http::StatusCode::BAD_REQUEST,
        )),
    }
}

pub async fn handle_rejection(
    err: Rejection,
) -> std::result::Result<impl warp::Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if err.find::<IncorrectPassword>().is_some() {
        Ok(warp::reply::with_status(
            "Incorrect password",
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else if err.find::<UserBanned>().is_some() {
        Ok(warp::reply::with_status(
            "User banned",
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else if err.find::<UserNotFound>().is_some() {
        Ok(warp::reply::with_status(
            "User not found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if err.find::<UserAlereadyExists>().is_some() {
        Ok(warp::reply::with_status(
            "User already exists",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if err.find::<WrongToken>().is_some() {
        Ok(warp::reply::with_status(
            "Wrong token",
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal server error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub fn post_json() -> impl Filter<Extract = (PostCreateRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn comment_json() -> impl Filter<Extract = (CommentCreateRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn login_json() -> impl Filter<Extract = (LoginRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn signup_json() -> impl Filter<Extract = (SignupRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn delete_json() -> impl Filter<Extract = (UserDeleteRequest,), Error = warp::Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn upgrade_json(
) -> impl Filter<Extract = (UserUpgradeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn ban_json() -> impl Filter<Extract = (UserBanRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn unban_json() -> impl Filter<Extract = (UserUnbanRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn react_json() -> impl Filter<Extract = (LikeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn unreact_json() -> impl Filter<Extract = (UnlikeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn display_name_change_json(
) -> impl Filter<Extract = (DisplayNameChangeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn description_change_json(
) -> impl Filter<Extract = (DescriptionChangeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn image_to_post_add_json(
) -> impl Filter<Extract = (AddImageToPostRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn set_pfp_json(
) -> impl Filter<Extract = (SetPFPRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
