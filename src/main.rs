mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await
}
