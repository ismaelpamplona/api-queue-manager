// src/rabbitmq.rs
use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties, Result};
use tracing::info;

pub async fn setup_rabbitmq() -> Result<Channel> {
    // Get the RabbitMQ connection string from the environment variable
    let addr = std::env::var("RABBITMQ_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@rabbitmq:5672".to_string());

    // Connect to RabbitMQ using the default connection properties
    let connection = Connection::connect(&addr, ConnectionProperties::default()).await?;

    info!("Connected to RabbitMQ");

    // Create a channel on the connection
    let channel = connection.create_channel().await?;

    // Declare a queue named "api_requests"
    channel
        .queue_declare(
            "api_requests", // Queue name
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Declared queue 'api_requests'");

    Ok(channel)
}
