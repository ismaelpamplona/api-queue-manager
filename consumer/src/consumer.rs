use futures_util::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Connection, Result,
};
use models::{ApiRequest, ApiResponse}; // Import from shared library
use reqwest::Client; // Import reqwest HTTP client
use serde_json::Value;
use tracing::info; // Import your models

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

    // Initialize an HTTP client
    let http_client = Client::new();

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                // Deserialize the message payload into ApiRequest
                let request: ApiRequest = match serde_json::from_slice(&delivery.data) {
                    Ok(req) => req,
                    Err(e) => {
                        eprintln!("Failed to deserialize message: {:?}", e);
                        delivery.ack(BasicAckOptions::default()).await?;
                        continue; // Skip processing this message
                    }
                };

                // Execute the HTTP request
                let response = execute_request(&http_client, request).await;

                // Log or handle the response as needed
                match response {
                    Ok(resp) => {
                        info!("Successfully processed request: {:?}", resp);
                        // Here you can send the response back to a response queue or handle it accordingly
                    }
                    Err(e) => {
                        eprintln!("Failed to process request: {:?}", e);
                    }
                }

                // Acknowledge the message after processing
                delivery.ack(BasicAckOptions::default()).await?;
            }
            Err(e) => {
                eprintln!("Error in consumer: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn execute_request(
    client: &Client,
    request: ApiRequest,
) -> Result<ApiResponse, reqwest::Error> {
    let url = format!("http://localhost:3000{}", request.endpoint); // Construct URL from endpoint

    let response = match request.method {
        // Match based on HTTP method type
        Method::GET => client.get(&url).send().await?,
        Method::POST => client.post(&url).json(&request.payload).send().await?,
        Method::PUT => client.put(&url).json(&request.payload).send().await?,
        Method::DELETE => client.delete(&url).send().await?,
        _ => {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::METHOD_NOT_ALLOWED,
                "Unsupported HTTP method",
            ));
        }
    };

    // Deserialize response into ApiResponse
    let status = response.status();
    let message = response.text().await.unwrap_or_default();

    Ok(ApiResponse { status, message })
}
