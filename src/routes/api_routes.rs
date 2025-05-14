use axum::{Router, response::Json, routing::get};
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            Json(json!({
                "status": "success",
                "message": "Hello, World!"
            }))
        }),
    )
}
