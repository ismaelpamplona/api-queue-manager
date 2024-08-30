use crate::models::{ApiRequest, ApiResponse};
use axum::{extract::Json, http::StatusCode};
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};
use serde_json;
use std::sync::Arc;

pub async fn handle_post_request(
    Json(payload): Json<ApiRequest>,
    rabbitmq_channel: Arc<Channel>, // Accept Arc<Channel> now
) -> Result<Json<ApiResponse>, StatusCode> {
    println!("Received POST request: {:?}", payload);

    // Serialize the payload to JSON
    let payload_json = match serde_json::to_string(&payload) {
        Ok(json) => json,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Publish the message to RabbitMQ
    if let Err(err) = rabbitmq_channel
        .basic_publish(
            "",             // Default exchange
            "api_requests", // Queue name
            BasicPublishOptions::default(),
            payload_json.as_bytes(), // Payload data
            BasicProperties::default(),
        )
        .await
    {
        println!("Failed to publish message to RabbitMQ: {:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        message: "POST request enqueued".to_string(),
    }))
}
