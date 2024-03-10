use std::time::SystemTime;

use tokio_rusqlite::{Connection, params};
use tracing::info;
use warp::reply::Json;

use crate::{get_token::get_token, types::{Post, SignupRequest}};

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

pub async fn check_banned(connection: &Connection, user_id: i64) -> bool {
    let query = "SELECT is_banned FROM users WHERE user_id = ?";

    if !check_user_id(connection, user_id).await {
        return true;
    }

    let is_banned = connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![user_id]).unwrap();
        if let Ok(Some(row)) = rows.next() {
            Ok(row.get::<_, i64>(0).unwrap() != 0)
        } else {
            Ok(true)
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
    
    let reactions_delete_query = "DELETE FROM reactions WHERE user_id = ?";
    connection.call(move |conn| {
        let mut statement = conn.prepare(reactions_delete_query).unwrap();
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

    info!(
        "Added post {} for user {}", 
        post.post_id, 
        post.user_id,
    );
}

pub async fn add_user_db(connection: &Connection, request: SignupRequest) -> Json {
    let user_id = max_user_id(connection).await.unwrap();
    let user_name = request.user_name.clone();
    
    let signup_query = "INSERT INTO users VALUES (:user_id, :user_name, :passwd, 0, 0)";
    connection.call(move |conn| {
        let mut statement = conn.prepare(signup_query).unwrap();
        statement.execute(params![user_id, request.user_name, request.passwd]).unwrap();
        Ok(0)
    }).await.unwrap();

    info!("User {} created with id {}", user_name, user_id);
    warp::reply::json(&get_token(user_id, 0))
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

pub async fn check_reaction(connection: &Connection, user_id: i64, post_id: i64, reaction_type: i64) -> bool {
    let query = "SELECT post_id FROM reactions WHERE type = ? AND user_id = ? AND post_id = ?";

    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        let mut rows = statement.query(params![reaction_type, user_id, post_id]).unwrap();
        if let Ok(Some(_)) = rows.next() {
            Ok(true)
        } else {
            Ok(false)
        }
    }).await.unwrap()
}

pub async fn add_reaction_db(connection: &Connection, user_id: i64, post_id: i64, reaction_type: i64) -> bool {
    let query = "INSERT INTO reactions VALUES (?, ?, ?)"; 

    if check_reaction(connection, user_id, post_id, reaction_type).await {
        info!("Reaction already exists");
        return true;
    }

    connection.call(move |conn| {
        let mut statement = conn.prepare(query).unwrap();
        statement.execute(params![reaction_type, user_id, post_id]).unwrap();
        Ok(0)
    }).await.unwrap();

    info!("Reaction {} added for post {} by user {}", reaction_type, post_id, user_id);
    false
}
