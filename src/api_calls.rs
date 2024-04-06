use crate::database_functions::*;
use crate::get_token::get_token;
use crate::types::*;
use crate::verify_token::{self, Claims};
use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use jsonwebtoken::TokenData;
use tokio_rusqlite::params;
use tracing::{error, info};
use warp::filters::multipart::FormData;
use warp::reject::Rejection;
use warp::Filter;

pub async fn get_posts_by_user(user_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT * FROM posts WHERE user_id = ?";

    if !check_user_id(&connection, user_id).await {
        let r = "User not found";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::NOT_FOUND,
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

    let query = "
        SELECT posts.post_id, posts.user_id, posts.date, posts.body 
        FROM posts 
        JOIN posts_tags
        ON posts.post_id = posts_tags.post_id 
        WHERE posts_tags.tag_id = ?
    ";

    let post_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![tag_id]).unwrap();
            let mut post_vec: Vec<Post> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                post_vec.push(Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap(),
                    likes: row.get(4).unwrap(),
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
    let query = "SELECT * FROM posts";

    let post_list = connection
        .call(move |conn| {
            let mut statement = conn.prepare(query).unwrap();
            let mut rows = statement.query(params![]).unwrap();
            let mut post_vec: Vec<Post> = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                post_vec.push(Post {
                    post_id: row.get(0).unwrap(),
                    user_id: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    body: row.get(3).unwrap(),
                    likes: row.get(4).unwrap(),
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

pub async fn get_post_by_id(post_id: i64) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let query = "SELECT * FROM posts WHERE post_id = ?";

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
                };
            } else {
                post = Post {
                    post_id: -1,
                    user_id: 0,
                    date: 0,
                    body: "".to_string(),
                    likes: 0,
                };
            }
            Ok(post)
        })
        .await
        .unwrap();

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
    let query = "SELECT user_id, user_name, display_name, description FROM users WHERE user_id = ?";

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
                profile = Profile {
                    user_id: row.get(0).unwrap(),
                    user_name: row.get(1).unwrap(),
                    display_name: row.get(2).unwrap(),
                    description: row.get(3).unwrap(),
                };
            } else {
                profile = Profile {
                    user_id: -1,
                    user_name: "".to_string(),
                    display_name: "".to_string(),
                    description: "".to_string(),
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
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();
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

    let images = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![post_id]).unwrap();
        let mut image_ids: Vec<String> = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            image_ids.push(row.get::<_, String>(0).unwrap());
        }
        Ok(image_ids)
    }).await.unwrap();

    let image_id_list = ImageList { image_list: images };
    Ok(warp::reply::with_status(
        warp::reply::json(&image_id_list),
        warp::http::StatusCode::OK
    ))
}

pub async fn post(request: PostCreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
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
            body: request.body,
            likes: 0,
        },
        request.tags,
    )
    .await;

    Ok(warp::reply::with_status(
        warp::reply::json(&post_count),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn react(request: LikeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

    if check_banned(&connection, token.claims.uid).await == true {
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

pub async fn login(request: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
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
                let r = "Password incorrect".to_string();
                Ok(warp::reply::with_status(
                    warp::reply::json(&r),
                    warp::http::StatusCode::UNAUTHORIZED,
                ))
            }
        }
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&e),
            warp::http::StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn signup(request: SignupRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();

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
            warp::http::StatusCode::CREATED,
        ))
    }
}

pub async fn delete_user(request: UserDeleteRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;
    if check_user_id(&connection, id).await {
        let delete_query = "UPDATE users SET is_banned=1 WHERE user_id = ?";
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

pub async fn upgrade_user(
    request: UserUpgradeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
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

pub async fn ban_user(request: UserBanRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
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

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = request.user_id;

    if check_user_id(&connection, id).await {
        let ban_query = "UPDATE users SET is_banned=1 WHERE user_id = ?";
        connection
            .call(move |conn| {
                let mut statement = conn.prepare(ban_query).unwrap();
                statement.execute(params![id]).unwrap();
                Ok(0)
            })
            .await
            .unwrap();

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

pub async fn change_display_name(
    request: DisplayNameChangeRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
    let id = token.claims.uid;

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

pub async fn upload_image(form: FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db")
        .await
        .unwrap();
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
            match add_image_db(&connection, format!("{}.{}", image_uuid, file_ending)).await {
                Ok(val) => {
                    tokio::fs::write(&file_name, value).await.map_err(|e| {
                        error!("error writing file: {}", e);
                        warp::reject::reject()
                    })?;
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

pub async fn add_image_to_post(request: AddImageToPostRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let connection = tokio_rusqlite::Connection::open("projekt-db").await.unwrap();

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

    let token: TokenData<Claims>;
    match verify_token::verify_token(request.token) {
        Ok(val) => token = val,
        Err(_) => {
            let r = "Wrong token";
            return Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::UNAUTHORIZED,
            ));
        }
    }

    if token.claims.uid != get_user_from_post(&connection, request.post_id).await && token.claims.is_admin == 0 {
        let r = "User not authorized";
        return Ok(warp::reply::with_status(
            warp::reply::json(&r),
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    match assign_image_to_post_db(&connection, request.post_id, request.image_id).await {
        Ok(_) => {
            let r = "Image added to post";
            Ok(warp::reply::with_status(
                warp::reply::json(&r),
                warp::http::StatusCode::OK,
            ))
        },
        Err(e) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&e),
                warp::http::StatusCode::BAD_REQUEST,
            ))

        }
    }
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl warp::Reply, std::convert::Infallible> {
    let (code, message) = if err.is_not_found() {
        (warp::http::StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (warp::http::StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        error!("unhandled error: {:?}", err);
        (
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
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

pub fn react_json() -> impl Filter<Extract = (LikeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn display_name_change_json(
) -> impl Filter<Extract = (DisplayNameChangeRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn image_to_post_add_json(
) -> impl Filter<Extract = (AddImageToPostRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
