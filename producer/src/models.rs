use axum::http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub method: Method,
    pub endpoint: String,
    pub payload: Option<serde_json::Value>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub status: StatusCode,
    pub message: String,
}
