use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use warp::Filter;
mod models;
mod routes;

fn handle_client(mut stream: TcpStream) {
    loop {
        let data = format!(
            "{}\n",
            serde_json::to_string(&routes::info::load()).unwrap()
        );
        if stream.write_all(data.as_bytes()).is_err() {
            println!("Client disconnected");
            break;
        }
        thread::sleep(Duration::from_secs(5)); // Wait 5 seconds
    }
}

async fn start_tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:9999").expect("Failed to bind to port");

    println!("Server running on 127.0.0.1:9999...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let system_info = warp::path("api")
        .and(warp::path("info"))
        .and(warp::get())
        .map(|| {
            warp::reply::with_status(
                warp::reply::json(&routes::info::load()),
                warp::http::StatusCode::OK,
            )
        });

    let routes = system_info;

    // Web API
    let warp_server = warp::serve(routes).run(([0, 0, 0, 0], 8000));

    // Run TCP and Warp servers concurrently
    tokio::select! {
        _ = tokio::spawn(start_tcp_server()) => (),
        _ = warp_server => (),
    }
}
