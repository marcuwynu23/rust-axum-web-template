use axum::Router;
use tracing_subscriber::EnvFilter;

use rust_axum_web_template::database::database;
use rust_axum_web_template::middlewares::logging_middleware::logging_middleware;
use rust_axum_web_template::routes::{api_routes, web_routes};

pub fn routes() -> Router {
    Router::new()
        .merge(web_routes::routes())
        .nest("/api", api_routes::routes())
        .layer(logging_middleware())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("trace")) // ✅ Set minimum log level to INFO
        .init();
    database::connect_db().await;

    // build our application with a route
    let app = routes();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
