mod handler;
mod rabbitmq;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize logging

    // Set up RabbitMQ channel using the rabbitmq module
    let rabbitmq_channel = match rabbitmq::setup_rabbitmq().await {
        Ok(channel) => Arc::new(channel),
        Err(e) => {
            eprintln!("Failed to connect to RabbitMQ: {:?}", e);
            return;
        }
    };

    info!("RabbitMQ setup completed successfully");

    // Create the router using the routes module, passing the RabbitMQ channel
    let app = routes::run(rabbitmq_channel.clone()); 

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
