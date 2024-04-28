use std::time::SystemTime;

use tokio_rusqlite::{Connection, params};
use tracing::info;


use crate::{get_token::get_token, types::{Post, SignupRequest}};
use crate::auth::*;

pub async fn check_user_id(connection: &Connection, id: i64) -> bool {
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

pub async fn check_user_name(connection: &Connection, name: String) -> bool {
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

pub async fn check_post(connection: &Connection, id: i64) -> bool {
    let query = "SELECT post_id FROM posts WHERE post_id = ?";
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

pub async fn check_image(connection: &Connection, id: i64) -> bool {
    let query = "SELECT image_id FROM images WHERE image_id = ?";
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

pub async fn check_post_image(connection: &Connection, image_id: i64, post_id: i64) -> bool {
    let query = "SELECT image_id FROM posts_images WHERE image_id = ? AND post_id = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([image_id, post_id]).unwrap();
        if let Some(_) = rows.next().unwrap() {
            Ok(true)
        } else {
            Ok(false)
        }
    }).await.unwrap()
}


pub async fn check_banned(connection: &Connection, user_id: i64) -> bool {
    let query = "SELECT is_active, expires_on FROM bans WHERE user_id = ? ORDER BY given_on DESC LIMIT 1";

    if !check_user_id(connection, user_id).await {
        return true;
    }
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let is_banned = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap();
        if let Ok(Some(row)) = rows.next() {
            Ok(row.get::<_, i64>(0).unwrap() == 1 && row.get::<_, i64>(1).unwrap() > timestamp)
        } else {
            Ok(false)
        }
    }).await.unwrap();

    is_banned
}

pub async fn purge_data(connection: &Connection, user_id: i64) {
    let query = "SELECT post_id FROM posts WHERE user_id = ?";
    let post_ids = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap();
        let mut post_id_vec: Vec<i64> = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            post_id_vec.push(row.get(0).unwrap());
        }
        Ok(post_id_vec)
    }).await.unwrap();

    let post_tag_delete_query = "DELETE FROM posts_tags WHERE post_id = ?";
    for post_id in post_ids.iter() {
        let post_id = *post_id;
        connection.call(move |conn| {
            let mut statement = conn.prepare(post_tag_delete_query).unwrap();
            statement.execute(params![post_id]).unwrap();
            Ok(0)
        }).await.unwrap();
    }
    
    let find_posts_query = "SELECT post_id FROM likes WHERE user_id = ?";
    let like_post_ids = connection.call(move |conn| {
        let mut statement = conn.prepare(find_posts_query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap();
        let mut post_id_vec: Vec<i64> = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            post_id_vec.push(row.get(0).unwrap());
        }
        Ok(post_id_vec)
    }).await.unwrap();

    let post_like_delete_query = "UPDATE posts SET likes=likes-1 WHERE post_id = ?";
    for post_id in like_post_ids.iter() {
        let post_id = *post_id;
        connection.call(move |conn| {
            let mut statement = conn.prepare(post_like_delete_query).unwrap();
            statement.execute(params![post_id]).unwrap();
            Ok(0)
        }).await.unwrap();
    };
    
    let likes_delete_query = "DELETE FROM likes WHERE user_id = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(likes_delete_query).unwrap();
        statement.execute(params![user_id]).unwrap();
        Ok(0)
    }).await.unwrap();

    let post_delete_query = "DELETE FROM posts WHERE user_id = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(post_delete_query).unwrap();
        statement.execute(params![user_id]).unwrap();
        Ok(0)
    }).await.unwrap();
}

pub async fn count_posts(connection: &Connection) -> Result<i64, &str> {
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

pub async fn count_users(connection: &Connection) -> Result<i64, &str> {
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

pub async fn max_user_id(connection: &Connection) -> Result<i64, &str> {
    let query = "SELECT MAX(user_id) FROM users";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            Ok(Ok(val.get::<_, i64>(0).unwrap() + 1))
        } else {
            Ok(Err("Failed to max user id"))
        }
    }).await.unwrap()
}

pub async fn count_tags(connection: &Connection) -> Result<i64, &str> {
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

// pub async fn get_tag_by_id(connection: &Connection, id: i64) -> Result<String, i8> {
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

pub async fn get_tag_by_name(connection: &Connection, name: String) -> Result<i64, i8> {
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

pub async fn get_user_from_post(connection: &Connection, id: i64) -> i64 {
    let query = "SELECT user_id FROM posts WHERE post_id = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query([id]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            Ok(val.get(0).unwrap())
        } else {
            Ok(-1)
        }
    }).await.unwrap()
}

pub async fn add_post_tag_db(connection: &Connection, post_id: i64, tag_id: i64) {
    let query = "INSERT INTO posts_tags VALUES (?, ?)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute([post_id, tag_id]).unwrap();
        Ok(0)
    }).await.unwrap();
}

pub async fn add_tag_db(connection: &Connection, name: String) -> i64 {
    let tag_count = count_tags(connection).await.unwrap(); 
    let query = "INSERT INTO tags VALUES (:tag_id, :tag_name)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![tag_count, name]).unwrap();
        Ok(0)
    }).await.unwrap();
    tag_count
}

pub async fn add_post_db(connection: &Connection, post: Post, tags: Vec<String>) {
    let time_since_epoch: i64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;

    let query = "INSERT INTO posts VALUES (?, ?, ?, ?, ?)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![post.post_id, post.user_id, time_since_epoch, post.body, 0]).unwrap();
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

    info!(
        "Added post {} for user {}", 
        post.post_id, 
        post.user_id,
    );
}

pub async fn add_user_db(connection: &Connection, request: SignupRequest) -> String {
    let user_id = max_user_id(connection).await.unwrap();
    let user_name = request.user_name.clone();
    let password = request.passwd.clone();
    let password_hash = get_hash(password);
    let signup_query = "INSERT INTO users VALUES (:user_id, :user_name, :user_name, '', :passwd, 0, '')";
    connection.call(move |conn| {
        let mut statement = conn.prepare(signup_query).unwrap();
        statement.execute(params![user_id, request.user_name, password_hash]).unwrap();
        Ok(0)
    }).await.unwrap();

    info!("User {} created with id {}", user_name, user_id);
    get_token(user_id, 0)
}

pub async fn get_id_passwd_adm(connection: &Connection, user: String) -> Result<(i64, String, i64), String> {
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

pub async fn check_like(connection: &Connection, user_id: i64, post_id: i64) -> bool {
    let query = "SELECT post_id FROM likes WHERE post_id = ? AND user_id = ?";

    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![post_id, user_id]).unwrap();
        if let Ok(Some(_)) = rows.next() {
            Ok(true)
        } else {
            Ok(false)
        }
    }).await.unwrap()
}

pub async fn add_like_db(connection: &Connection, user_id: i64, post_id: i64) -> bool {
    let query = "INSERT INTO likes VALUES (?, ?)"; 
    let update_query = "UPDATE posts SET likes=likes+1 WHERE post_id = ?";

    if check_like(connection, user_id, post_id).await {
        info!("Like already exists");
        return true;
    }

    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![user_id, post_id]).unwrap();
        Ok(0)
    }).await.unwrap();
    
    connection.call(move |conn| {
        let mut statement = conn.prepare(update_query).unwrap();
        statement.execute(params![post_id]).unwrap();
        Ok(0)
    }).await.unwrap();

    info!("Like added for post {} by user {}", post_id, user_id);
    false
}

pub async fn remove_like_db(connection: &Connection, user_id: i64, post_id: i64) -> bool {
    let query = "DELETE FROM likes WHERE user_id=? AND post_id=?"; 
    let update_query = "UPDATE posts SET likes=likes-1 WHERE post_id = ?";

    if !check_like(connection, user_id, post_id).await {
        info!("Like doesn't exists");
        return true;
    }

    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![user_id, post_id]).unwrap();
        Ok(0)
    }).await.unwrap();
    
    connection.call(move |conn| {
        let mut statement = conn.prepare(update_query).unwrap();
        statement.execute(params![post_id]).unwrap();
        Ok(0)
    }).await.unwrap();

    info!("Like added for post {} by user {}", post_id, user_id);
    false
}

pub async fn max_image_id(connection: &Connection) -> i64 {
    let query = "SELECT MAX(image_id) FROM images";
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            let ret = match val.get::<_, i64>(0) {
                Ok(val) => {val + 1},
                Err(_) => {0}
            };
            Ok(ret)
        } else {
            Ok(0)
        }
    }).await.unwrap()
}

pub async fn add_image_db(connection: &Connection, image_file: String) -> Result<i64, &str> {
    let image_query = "INSERT INTO images VALUES (?, ?)";

    let image_count = max_image_id(connection).await;

    connection.call(move |conn| {
        let mut statement = conn.prepare(image_query).unwrap();
        statement.execute(params![image_count, image_file]).unwrap();
        Ok(0)
    }).await.unwrap();
    
    Ok(image_count)
}

pub async fn assign_image_to_post_db(connection: &Connection, post_id: i64, image_id: i64) -> Result<(), &str> {
    if check_post_image(connection, image_id, post_id).await {
        return Err("Image already added to this post");
    }

    let image_query = "INSERT INTO posts_images VALUES (?, ?)";

    connection.call(move |conn| {
        let mut statement = conn.prepare(image_query).unwrap();
        statement.execute(params![post_id, image_id]).unwrap();
        Ok(0)
    }).await.unwrap();
    
    Ok(())
}

pub async fn assign_image_to_user(connection: &Connection, user_id: i64, image_id: i64) -> Result<(), &str> {
    let image_query = "UPDATE users SET pfp_id=? WHERE user_id=?";

    connection.call(move |conn| {
        let mut statement = conn.prepare(image_query).unwrap();
        statement.execute(params![image_id, user_id]).unwrap();
        Ok(0)
    }).await.unwrap();
    
    Ok(())
}

pub async fn add_upload_db(connection: &Connection, user_id: i64, weight: i16) {
    let time_since_epoch: i64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let add_query = "INSERT INTO uploads VALUES (?, ?, ?)";

    connection.call(move |conn| {
        let mut statement = conn.prepare(add_query).unwrap();
        statement.execute(params![user_id, weight, time_since_epoch]).unwrap();
        Ok(0)
    }).await.unwrap();
}

pub async fn get_upload(connection: &Connection, user_id: i64) -> i64 {
    let mut time_since_epoch: i64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    time_since_epoch -= 60;
    let query = "SELECT SUM(weight) FROM uploads WHERE user_id = ? AND date > ?";
    
    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id, time_since_epoch]).unwrap();
        if let Some(val) = rows.next().unwrap() {
            let ret = match val.get::<_, i64>(0) {
                Ok(val) => {val},
                Err(_) => {0}
            };
            Ok(ret)
        } else {
            Ok(0)
        }

    }).await.unwrap()
}

pub async fn is_limited(connection: &Connection, user_id: i64) -> bool {
    get_upload(connection, user_id).await > 50   
}
