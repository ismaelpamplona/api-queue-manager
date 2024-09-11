use crate::handler::handle_message;
use futures_util::stream::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Connection};
use std::sync::Arc;

pub async fn start_consumer(connection: Connection) -> lapin::Result<()> {
    let channel = Arc::new(connection.create_channel().await?); // Wrap the channel in Arc to share between threads
    let mut consumer = channel
        .basic_consume(
            "api_requests",          // Queue name
            "api_request_processor", // Consumer tag
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut request_number = 1;

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                // Pass the shared rabbitmq_channel (Arc<Channel>) to handle_message
                if let Ok(should_increment) =
                    handle_message(delivery, request_number, Arc::clone(&channel)).await
                {
                    if should_increment {
                        request_number += 1; // Increment request number only if request is triggered
                    }
                } else {
                    eprintln!("Error handling message.");
                }
            }
            Err(e) => {
                eprintln!("Error in consumer: {:?}", e);
            }
        }
    }

    Ok(())
}
