use crate::handler::handle_message;
use futures_util::stream::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Connection};

pub async fn start_consumer(connection: Connection) -> lapin::Result<()> {
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
                if let Err(e) = handle_message(delivery).await {
                    eprintln!("Error handling message: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Error in consumer: {:?}", e);
            }
        }
    }

    Ok(())
}
