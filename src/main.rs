use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::interval;
use warp::Filter;
use warp::ws::{Message, WebSocket};

mod models;
mod routes;

// WebSocket handler
async fn handle_websocket(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    let mut interval = interval(Duration::from_secs(3));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                match serde_json::to_string(&routes::info::load()) {
                    Ok(data) => {
                        if tx.send(Message::text(data)).await.is_err() {
                            eprintln!("WebSocket client disconnected");
                            break;
                        }
                    }
                    Err(e) => eprintln!("Failed to serialize system info: {}", e),
                }
            }
            Some(Ok(msg)) = rx.next() => {
                println!("Received message from client: {:?}", msg);
            }
            else => {
                eprintln!("WebSocket connection closed");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // WebSocket route
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_websocket));

    // REST API route
    let system_info_route = warp::path!("api" / "info")
        .and(warp::get())
        .map(|| {
            match serde_json::to_string(&routes::info::load()) {
                Ok(json) => warp::reply::with_status(
                    warp::reply::json(&json),
                    warp::http::StatusCode::OK,
                ),
                Err(e) => {
                    eprintln!("Failed to serialize system info: {}", e);
                    warp::reply::with_status(
                        warp::reply::json(&"Internal Server Error"),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                }
            }
        });

    // Static file serving route
    let static_files_route = warp::fs::dir("./static");

    // Combine all routes
    let routes = websocket_route.or(system_info_route).or(static_files_route);

    // Start the Warp server
    let address = ([0, 0, 0, 0], 8000);
    println!("Starting server on http://{}:{}", "localhost", address.1);
    warp::serve(routes).run(address).await;
}
