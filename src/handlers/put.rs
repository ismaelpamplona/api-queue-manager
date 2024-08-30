use crate::models::{ApiRequest, ApiResponse};
use axum::extract::Json;

pub async fn handle_put_request(Json(payload): Json<ApiRequest>) -> Json<ApiResponse> {
    println!("Received PUT request: {:?}", payload);
    Json(ApiResponse {
        status: "success".to_string(),
        message: "PUT request received".to_string(),
    })
}
