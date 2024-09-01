mod handlers;
mod models;
mod rabbitmq;
mod routes; // Import the rabbitmq module

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize logging

    // Set up RabbitMQ channel using the rabbitmq module
    let rabbitmq_channel = match rabbitmq::setup_rabbitmq().await {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to connect to RabbitMQ: {:?}", e);
            return;
        }
    };

    // Log RabbitMQ setup success
    info!("RabbitMQ setup completed successfully");

    // Create the router using the routes module, passing the RabbitMQ channel
    let app = routes::run(rabbitmq_channel);

    // Run the server on 0.0.0.0:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
