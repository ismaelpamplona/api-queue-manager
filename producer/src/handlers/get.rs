use crate::models::ApiResponse;
use axum::{http::StatusCode, Json};

pub async fn handle_get_request() -> Result<Json<ApiResponse>, StatusCode> {
    println!("Received GET request");

    // This is where you could add logic to fetch data or query state
    Ok(Json(ApiResponse {
        status: "success".to_string(),
        message: "GET request received".to_string(),
    }))
}
