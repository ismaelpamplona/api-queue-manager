use crate::models::{ApiRequest, ApiResponse};
use axum::extract::Json;

pub async fn handle_delete_request(Json(payload): Json<ApiRequest>) -> Json<ApiResponse> {
    println!("Received DELETE request: {:?}", payload);
    Json(ApiResponse {
        status: "success".to_string(),
        message: "DELETE request received".to_string(),
    })
}
