use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicNackOptions, BasicPublishOptions};
use lapin::BasicProperties;
use lapin::Channel;
use models::{ApiRequest, ApiResponse};
use reqwest::{Client, Method, StatusCode};
use serde_json::{from_slice, to_vec};
use std::result::Result;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::info;

pub async fn handle_message(
    delivery: Delivery,
    request_number: usize,
    rabbitmq_channel: Arc<Channel>,
) -> Result<bool, String> {
    // Deserialize the message payload into ApiRequest
    let mut request: ApiRequest = match from_slice(&delivery.data) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to deserialize message: {:?}", e);
            // Acknowledge the message even if deserialization fails
            delivery
                .ack(BasicAckOptions::default())
                .await
                .map_err(|err| err.to_string())?;
            return Err("Failed to deserialize message".to_string());
        }
    };

    // Initialize or increment retry count
    let retry_count = request.retry_count.unwrap_or(0);

    // Log the retry count for debugging
    info!(
        "Processing request number: {}, Retry count: {}",
        request_number, retry_count
    );

    // Make the HTTP request
    let http_client = Client::new();
    let res = match execute_request(&http_client, &request).await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to process request: {:?}", e);
            // Acknowledge the message even if request execution fails
            delivery
                .ack(BasicAckOptions::default())
                .await
                .map_err(|err| err.to_string())?;
            return Err(e);
        }
    };

    // Handle the response, checking for rate limit issues
    if res.status == StatusCode::TOO_MANY_REQUESTS || res.status == StatusCode::FORBIDDEN {
        // If rate limit is hit, introduce a delay before retrying based on retry_count
        let delay_seconds = match retry_count {
            0 => 5,  // First retry after 5 seconds
            1 => 10, // Second retry after 10 seconds
            2 => 20, // Third retry after 20 seconds
            _ => 60, // Beyond third retry, wait 60 seconds
        };

        eprintln!(
            " >>> ⏳ Rate limit hit for request number: {}. Waiting {} seconds before retrying...\n",
            request_number, delay_seconds
        );

        // Wait for the determined delay before retrying
        sleep(Duration::from_secs(delay_seconds)).await;

        // Increment retry count and update the message before republishing it
        request.retry_count = Some(retry_count + 1);

        // Serialize the request with the updated retry count
        let payload_with_retry_count = match to_vec(&request) {
            Ok(payload) => payload,
            Err(e) => {
                eprintln!("Failed to serialize message with retry count: {:?}", e);
                return Err("Serialization failed".to_string());
            }
        };

        // Log the retry count before republishing
        info!(
            "Republishing message with retry count: {}, request number: {}",
            retry_count + 1,
            request_number
        );

        // Republish the message with the updated retry count
        rabbitmq_channel
            .basic_publish(
                "",             // Use the default exchange
                "api_requests", // Publish to the same queue
                BasicPublishOptions::default(),
                &payload_with_retry_count, // New payload with updated retry count
                BasicProperties::default(),
            )
            .await
            .map_err(|err| err.to_string())?;

        // NACK the original message (without requeueing)
        delivery
            .nack(BasicNackOptions {
                requeue: false, // Do not requeue the original message since we republished it
                ..Default::default()
            })
            .await
            .map_err(|err| err.to_string())?;

        return Ok(false); // Return false since the request was not processed due to rate limiting
    } else {
        info!(
            "\n\n >>> ✅ Successfully processed request number: {} \n >>> 📨 Response: {:?} \n",
            request_number, res
        );

        // Acknowledge the message after successful processing
        delivery
            .ack(BasicAckOptions::default())
            .await
            .map_err(|err| err.to_string())?;

        return Ok(true); // Return true since the request was successfully processed
    }
}

pub async fn execute_request(client: &Client, request: &ApiRequest) -> Result<ApiResponse, String> {
    let url = &request.endpoint;

    let fake_request_id = format!("request-1");

    let res = match request.method {
        Method::GET => {
            client
                .get(url)
                .header("X-Request-ID", fake_request_id) // Simulating different clients
                .send()
                .await
        }
        Method::POST => {
            client
                .post(url)
                .json(&request.payload)
                .header("X-Request-ID", fake_request_id)
                .send()
                .await
        }
        Method::PUT => {
            client
                .put(url)
                .json(&request.payload)
                .header("X-Request-ID", fake_request_id)
                .send()
                .await
        }
        Method::DELETE => {
            client
                .delete(url)
                .header("X-Request-ID", fake_request_id)
                .send()
                .await
        }
        _ => {
            return Err("Unsupported HTTP method".to_string());
        }
    };

    // Check for errors from the HTTP request
    let res = match res {
        Ok(res) => res,
        Err(err) => return Err(format!("HTTP request error: {:?}", err)),
    };

    // Deserialize response into ApiResponse
    let status = res.status();
    let message = res.text().await.unwrap_or_default();

    Ok(ApiResponse { status, message })
}
