use crate::handlers;
use warp::Filter;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_post = warp::post()
        .and(warp::path("posts"))
        .and(warp::path("get"))
        .and(warp::path::)
}

