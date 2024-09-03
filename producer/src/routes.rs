use crate::handler::handle_request;
use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use lapin::Channel;
use std::sync::Arc; // Assume a combined handler as described

pub fn run(rabbitmq_channel: Arc<Channel>) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Welcome to the API Queue Manager! 🦀" }),
        )
        .route(
            "/request",
            get({
                let rabbitmq_channel = rabbitmq_channel.clone();
                move |req| handle_request(Method::GET, req, rabbitmq_channel)
            }),
        )
        .route(
            "/request",
            post({
                let rabbitmq_channel = rabbitmq_channel.clone();
                move |req| handle_request(Method::POST, req, rabbitmq_channel)
            }),
        )
        .route(
            "/request",
            put({
                let rabbitmq_channel = rabbitmq_channel.clone();
                move |req| handle_request(Method::PUT, req, rabbitmq_channel)
            }),
        )
        .route(
            "/request",
            delete({
                let rabbitmq_channel = rabbitmq_channel.clone();
                move |req| handle_request(Method::DELETE, req, rabbitmq_channel)
            }),
        )
}
