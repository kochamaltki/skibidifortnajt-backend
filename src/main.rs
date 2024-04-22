use warp::Filter;
pub mod get_token;
pub mod get_secret;
pub mod verify_token;
pub mod api_calls;
pub mod types;
pub mod database_functions;
use crate::api_calls::*;


pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_posts_by_user = warp::get()
        .and(warp::path!("api" / "get" / "posts" / "by-user" / i64))
        .and_then(get_posts_by_user);
    
    let get_posts_by_tag = warp::get()
        .and(warp::path!("api" / "get" / "posts" / "by-tag" / String))
        .and_then(get_posts_by_tag);

    let get_tags_from_post = warp::get()
        .and(warp::path!("api" / "get" / "tags" / "from-post" / i64))
        .and_then(get_tags_from_post);

    let get_post_by_id = warp::get()
        .and(warp::path!("api" / "get" / "posts" / "by-id" / i64))
        .and_then(get_post_by_id);

    let get_posts = warp::get()
        .and(warp::path!("api" / "get" / "posts" / "all"))
        .and_then(get_posts);
    
    let get_user_name = warp::get()
        .and(warp::path!("api" / "get" / "user" / "name" / i64))
        .and_then(get_user_name);
    
    let get_user_id = warp::get()
        .and(warp::path!("api" / "get" / "user" / "id" / String))
        .and_then(get_user_id);

    let get_profile_by_id = warp::get()
        .and(warp::path!("api" / "get" / "profile" / "by-id" / i64))
        .and_then(get_profile_by_id);

    let get_images_from_post = warp::get()
        .and(warp::path!("api" / "get" / "images" / "from-post" / i64))
        .and_then(get_images_from_post);

    let get_image = warp::get()
        .and(warp::path!("api" / "get" / "image" / ..))
        .and(warp::fs::dir("./media/images"));
    
    let get_like_from_post_by_user = warp::get()
        .and(warp::path!("api" / "get" / "like" / i64 / i64))
        .and_then(get_like_from_post_by_user);

    // let get_posts_from_search = warp::get()
    //     .and(warp::path!("api" / "get" / "posts" / "from-search" / String))
    //     .and_then(get_posts_from_search);

    // let get_users_from_search = warp::get()
    //     .and(warp::path("api" / "get" / "users" / "from-search" / String))
    //     .and_then(get_posts_from_search);

    let post = warp::post()
        .and(warp::path!("api" / "post" / "add-post"))
        .and(warp::cookie::<String>("token"))
        .and(post_json())
        .and_then(post);

    let react = warp::post()
        .and(warp::path!("api" / "post" / "react"))
        .and(warp::cookie::<String>("token"))
        .and(react_json())
        .and_then(react);
    
    let unreact = warp::post()
        .and(warp::path!("api" / "post" / "unreact"))
        .and(warp::cookie::<String>("token"))
        .and(unreact_json())
        .and_then(unreact);

    let login = warp::post()
        .and(warp::path!("api" / "post" / "login")) 
        .and(login_json())
        .and_then(login);

    let signup = warp::post()
        .and(warp::path!("api" / "post" / "signup"))
        .and(signup_json())
        .and_then(signup);
    
    let delete = warp::post()
        .and(warp::path!("api" / "post" / "delete-user"))
        .and(warp::cookie::<String>("token"))
        .and(delete_json())
        .and_then(delete_user);

    let upgrade = warp::post()
        .and(warp::path!("api" / "admin" / "post" / "upgrade-user"))
        .and(warp::cookie::<String>("token"))
        .and(upgrade_json())
        .and_then(upgrade_user);

    let ban = warp::post()
        .and(warp::path!("api" / "admin" / "post" / "ban-user"))
        .and(warp::cookie::<String>("token"))
        .and(ban_json())
        .and_then(ban_user);

    let unban = warp::post()
        .and(warp::path!("api" / "admin" / "post" / "unban-user"))
        .and(warp::cookie::<String>("token"))
        .and(unban_json())
        .and_then(unban_user);

    let change_display_name = warp::post()
        .and(warp::path!("api" / "post" / "change" / "display-name"))
        .and(warp::cookie::<String>("token"))
        .and(display_name_change_json())
        .and_then(change_display_name);

    let change_description = warp::post()
        .and(warp::path!("api" / "post" / "change" / "description"))
        .and(warp::cookie::<String>("token"))
        .and(description_change_json())
        .and_then(change_description);

    let upload_image = warp::post()
        .and(warp::path!("api" / "post" / "upload" / "image")) // test
        .and(warp::cookie::<String>("token"))
        .and(warp::multipart::form().max_length(25000000))
        .and_then(upload_image);
    
    let add_image_to_post = warp::post()
        .and(warp::path!("api" / "post" / "add-image-to-post"))
        .and(warp::cookie::<String>("token"))
        .and(image_to_post_add_json())
        .and_then(add_image_to_post);

    get_posts_by_user
        .or(post)
        .or(get_posts)
        .or(login)
        .or(signup)
        .or(get_user_name)
        .or(delete)
        .or(get_user_id)
        .or(upgrade)
        .or(get_post_by_id)
        .or(ban)
        .or(unban)
        .or(get_posts_by_tag)
        .or(get_tags_from_post)
        .or(react)
        .or(get_profile_by_id)
        .or(get_images_from_post)
        .or(get_like_from_post_by_user)
        .or(change_display_name)
        .or(change_description)
        .or(upload_image)
        .or(get_image)
        .or(add_image_to_post)
        .or(unreact)
        // .or(get_posts_from_search)
        // .or(get_users_from_search)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = warp::cors().allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
        .allow_headers(vec!["content-type", "Access-Control-Allow-Origin"])
        .allow_credentials(true);

    let routes = routes().with(cors).recover(handle_rejection);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
