use crate::models::{ApiRequest, ApiResponse};
use axum::extract::Json;

pub async fn handle_post_request(Json(payload): Json<ApiRequest>) -> Json<ApiResponse> {
    println!("Received POST request: {:?}", payload);
    Json(ApiResponse {
        status: "success".to_string(),
        message: "POST request received".to_string(),
    })
}
