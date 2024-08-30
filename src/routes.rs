// src/routes.rs
use crate::handlers::{
    delete::handle_delete_request, get::handle_get_request, post::handle_post_request,
    put::handle_put_request,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub fn run(tx: mpsc::Sender<crate::models::ApiRequest>) -> Router {
    let state = Arc::new(tx);

    Router::new()
        .route(
            "/",
            get(|| async { "Welcome to the API Queue Manager! 🦀" }),
        )
        .route("/request", post(handle_post_request))
        .route("/request", get(handle_get_request))
        .route("/request", put(handle_put_request))
        .route("/request", delete(handle_delete_request))
        .with_state(state)
}
