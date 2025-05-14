use crate::controllers::web::controller;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(controller::home))
}
