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
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-user"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_posts_by_user);
    
    let get_posts_by_tag = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-tag"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_posts_by_tag);

    let get_tags_from_post = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("tags"))
        .and(warp::path("from-post"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_tags_from_post);

    let get_post_by_id = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("by-id"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_post_by_id);

    let get_posts = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("posts"))
        .and(warp::path("all"))
        .and(warp::path::end())
        .and_then(get_posts);
    
    let get_user_name = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("user"))
        .and(warp::path("name"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_user_name);
    
    let get_user_id = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("user"))
        .and(warp::path("id"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_user_id);

    let get_profile_by_id = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("profile"))
        .and(warp::path("by-id"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(get_profile_by_id);

    let get_images_from_post = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("images"))
        .and(warp::path("from-post"))
        .and(warp::path::param())
        .and_then(get_images_from_post);

    let get_image = warp::get()
        .and(warp::path("api"))
        .and(warp::path("get"))
        .and(warp::path("image"))
        .and(warp::fs::dir("./media/images"));

    // let get_posts_from_search = warp::get()
    //     .and(warp::path("api"))
    //     .and(warp::path("get"))
    //     .and(warp::path("posts"))
    //     .and(warp::path("from-search"))
    //     .and(warp::path::param())
    //     .and(warp::path::end())
    //     .and_then(get_posts_from_search);

    // let get_users_from_search = warp::get()
    //     .and(warp::path("api"))
    //     .and(warp::path("get"))
    //     .and(warp::path("users"))
    //     .and(warp::path("from-search"))
    //     .and(warp::path::param())
    //     .and(warp::path::end())
    //     .and_then(get_posts_from_search);

    let post = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("add-post"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(post_json())
        .and_then(post);

    let react = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("react"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(react_json())
        .and_then(react);

    let login = warp::post()
        .and(warp::path("api")) 
        .and(warp::path("post"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(login_json())
        .and_then(login);

    let signup = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("signup"))
        .and(warp::path::end())
        .and(signup_json())
        .and_then(signup);
    
    let delete = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("delete-user"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(delete_json())
        .and_then(delete_user);

    let upgrade = warp::post()
        .and(warp::path("api"))
        .and(warp::path("admin"))
        .and(warp::path("post"))
        .and(warp::path("upgrade-user"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(upgrade_json())
        .and_then(upgrade_user);

    let ban = warp::post()
        .and(warp::path("api"))
        .and(warp::path("admin"))
        .and(warp::path("post"))
        .and(warp::path("ban-user"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(ban_json())
        .and_then(ban_user);

    let unban = warp::post()
        .and(warp::path("api"))
        .and(warp::path("admin"))
        .and(warp::path("post"))
        .and(warp::path("unban-user"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(unban_json())
        .and_then(unban_user);

    let change_display_name = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("change"))
        .and(warp::path("display-name"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(display_name_change_json())
        .and_then(change_display_name);

    let change_description = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("change"))
        .and(warp::path("description"))
        .and(warp::path::end())
        .and(warp::cookie::<String>("token"))
        .and(description_change_json())
        .and_then(change_description);

    let upload_image = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("upload"))
        .and(warp::path("image"))
        .and(warp::cookie::<String>("token"))
        .and(warp::multipart::form().max_length(25000000))
        .and_then(upload_image);
    
    let add_image_to_post = warp::post()
        .and(warp::path("api"))
        .and(warp::path("post"))
        .and(warp::path("add-image-to-post"))
        .and(warp::path::end())
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
        .or(change_display_name)
        .or(change_description)
        .or(upload_image)
        .or(get_image)
        .or(add_image_to_post)
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

    let routes = routes().with(cors);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
