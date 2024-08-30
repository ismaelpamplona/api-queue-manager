mod handlers;
mod models;
mod routes;

use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::mpsc};

#[tokio::main]
async fn main() {
    let (tx, _rx) = mpsc::channel::<models::ApiRequest>(100);

    // Create the router using the routes module
    let app = routes::run(tx);

    // Run the server on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
