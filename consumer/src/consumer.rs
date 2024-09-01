use futures_util::stream::StreamExt; // Correct import for futures-util
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions}, // Correct import for BasicAckOptions
    types::FieldTable,
    Connection,
    Result,
};
use tracing::info;

pub async fn start_consumer(connection: Connection) -> Result<()> {
    let channel = connection.create_channel().await?;

    let mut consumer = channel
        .basic_consume(
            "api_requests",          // Queue name
            "api_request_processor", // Consumer tag
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                // Process the message
                info!("Received message: {:?}", delivery);

                // Acknowledge the message
                delivery.ack(BasicAckOptions::default()).await?;
            }
            Err(e) => {
                eprintln!("Error in consumer: {:?}", e);
            }
        }
    }

    Ok(())
}
