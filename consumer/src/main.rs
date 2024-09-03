mod consumer;
mod handler;

use consumer::start_consumer;
use lapin::{Connection, ConnectionProperties};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Set up a connection to RabbitMQ
    let addr = std::env::var("RABBITMQ_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@rabbitmq:5672".to_string());

    let connection = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .unwrap();

    info!("Connected to RabbitMQ");

    // Start the consumer
    if let Err(e) = start_consumer(connection).await {
        eprintln!("Failed to start consumer: {:?}", e);
    }
}
