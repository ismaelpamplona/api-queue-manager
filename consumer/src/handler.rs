use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use models::{ApiRequest, ApiResponse};
use reqwest::{Client, Method};
use serde_json::from_slice;
use std::result::Result;
use tracing::info;

pub async fn handle_message(delivery: Delivery, request_number: usize) -> Result<(), String> {
    // Deserialize the message payload into ApiRequest
    let request: ApiRequest = match from_slice(&delivery.data) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to deserialize message: {:?}", e);
            // Acknowledge the message even if deserialization fails
            delivery
                .ack(BasicAckOptions::default())
                .await
                .map_err(|err| err.to_string())?; // Convert lapin::Error to String
            return Err("Failed to deserialize message".to_string());
        }
    };

    let http_client = Client::new();

    // Pass the request number to print it instead of the response
    match execute_request(&http_client, request, request_number).await {
        Ok(resp) => {
            info!(
                "\n\n Successfully processed request number: {} \n\n Response: {:?} \n\n",
                request_number, resp
            );
        }
        Err(e) => {
            eprintln!("Failed to process request: {:?}", e);
        }
    }

    // Acknowledge the message after processing
    delivery
        .ack(BasicAckOptions::default())
        .await
        .map_err(|err| err.to_string())?; // Convert lapin::Error to String

    Ok(())
}

pub async fn execute_request(
    client: &Client,
    request: ApiRequest,
    request_number: usize,
) -> Result<ApiResponse, String> {
    let url = request.endpoint;

    // Generate a constant or reusable X-Request-ID
    let fake_request_id = format!("request-1"); // Use the same request ID for all

    let response = match request.method {
        Method::GET => {
            client
                .get(&url)
                .header("X-Request-ID", fake_request_id) // Simulating different clients
                .send()
                .await
        }
        Method::POST => {
            client
                .post(&url)
                .json(&request.payload)
                .header("X-Request-ID", fake_request_id)
                .send()
                .await
        }
        Method::PUT => {
            client
                .put(&url)
                .json(&request.payload)
                .header("X-Request-ID", fake_request_id)
                .send()
                .await
        }
        Method::DELETE => {
            client
                .delete(&url)
                .header("X-Request-ID", fake_request_id)
                .send()
                .await
        }
        _ => {
            return Err("Unsupported HTTP method".to_string());
        }
    };

    // Check for errors from the HTTP request
    let response = match response {
        Ok(res) => res,
        Err(err) => return Err(format!("HTTP request error: {:?}", err)),
    };

    // Deserialize response into ApiResponse
    let status = response.status();
    let message = response.text().await.unwrap_or_default();

    Ok(ApiResponse { status, message })
}
