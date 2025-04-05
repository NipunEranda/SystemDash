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

    // Periodically send system info to the client
    let mut interval = interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let data = serde_json::to_string(&routes::info::load()).unwrap();
                if tx.send(Message::text(data)).await.is_err() {
                    println!("WebSocket client disconnected");
                    break;
                }
            }
            Some(Ok(msg)) = rx.next() => {
                println!("Received message from client: {:?}", msg);
            }
            else => break,
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
    let system_info = warp::path("api")
        .and(warp::path("info"))
        .and(warp::get())
        .map(|| {
            warp::reply::with_status(
                warp::reply::json(&routes::info::load()),
                warp::http::StatusCode::OK,
            )
        });

    let static_files = warp::fs::dir("./static");

    let routes = websocket_route.or(system_info).or(static_files);

    // Start the Warp server
    println!("Starting server on http://localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
