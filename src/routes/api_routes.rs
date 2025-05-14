use crate::controllers::api::controller;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(controller::home))
}
