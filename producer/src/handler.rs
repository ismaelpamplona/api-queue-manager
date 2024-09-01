use axum::{
    extract::Json,
    http::{Method, StatusCode},
};
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};
use models::{ApiRequest, ApiResponse};
use serde_json;
use std::sync::Arc;

pub async fn handle_request(
    method: Method,
    Json(payload): Json<ApiRequest>,
    rabbitmq_channel: Arc<Channel>,
) -> Result<Json<ApiResponse>, StatusCode> {
    println!("Received {:?} request: {:?}", method, payload);

    let payload_json = match serde_json::to_string(&payload) {
        Ok(json) => json,
        Err(_) => {
            println!("Failed to serialize payload to JSON");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if let Err(err) = rabbitmq_channel
        .basic_publish(
            "",             // Default exchange
            "api_requests", // Queue name
            BasicPublishOptions::default(),
            payload_json.as_bytes(),
            BasicProperties::default(),
        )
        .await
    {
        println!("Failed to publish message to RabbitMQ: {:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(ApiResponse {
        status: StatusCode::ACCEPTED, // Use StatusCode::ACCEPTED to indicate request is queued for processing
        message: format!("{:?} request has been enqueued for processing.", method),
    }))
}
