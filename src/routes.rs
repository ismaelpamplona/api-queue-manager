use crate::handlers::{
    delete::handle_delete_request, get::handle_get_request, post::handle_post_request,
    put::handle_put_request,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use lapin::Channel;
use std::sync::Arc;

pub fn run(rabbitmq_channel: Channel) -> Router {
    // Use Arc to allow shared ownership of the RabbitMQ channel across multiple handlers
    let rabbitmq_channel = Arc::new(rabbitmq_channel);

    Router::new()
        .route(
            "/",
            get(|| async { "Welcome to the API Queue Manager! 🦀" }),
        )
        .route(
            "/request",
            post({
                let channel = Arc::clone(&rabbitmq_channel);
                move |req| handle_post_request(req, channel)
            }),
        )
        .route("/request", get(handle_get_request))
        .route(
            "/request",
            put({
                let channel = Arc::clone(&rabbitmq_channel);
                move |req| handle_put_request(req, channel)
            }),
        )
        .route(
            "/request",
            delete({
                let channel = Arc::clone(&rabbitmq_channel);
                move |req| handle_delete_request(req, channel)
            }),
        )
}
