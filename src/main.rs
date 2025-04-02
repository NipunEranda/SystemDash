use warp::Filter;

mod routes;
mod models;

#[tokio::main]
async fn main() {
    let hello = warp::path("info")
        .and(warp::get())
        .map(|| warp::reply::with_status(warp::reply::json(&routes::info::load()), warp::http::StatusCode::OK));

    let routes = hello;

    println!("Server running at http://localhost:5000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
