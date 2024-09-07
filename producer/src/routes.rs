use crate::handler::handle_request;
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use lapin::Channel;
use std::sync::Arc;

pub fn run(rabbitmq_channel: Arc<Channel>) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Welcome to the API Queue Manager! 🦀" }),
        )
        .route(
            "/request",
            post({
                let rabbitmq_channel = rabbitmq_channel.clone();
                move |req| handle_request(Method::POST, req, rabbitmq_channel)
            }),
        )
}
