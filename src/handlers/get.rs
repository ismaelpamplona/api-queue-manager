use crate::models::ApiResponse;
use axum::Json;

pub async fn handle_get_request() -> Json<ApiResponse> {
    println!("Received GET request");
    Json(ApiResponse {
        status: "success".to_string(),
        message: "GET request received".to_string(),
    })
}
