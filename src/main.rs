#[cfg(feature = "ssr")]
use axum::{response::IntoResponse, routing::get, Router};
#[cfg(feature = "ssr")]
use tower_http::services::ServeDir;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/hello", get(hello_handler))
        .fallback_service(ServeDir::new("dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(feature = "ssr")]
async fn hello_handler() -> impl IntoResponse {
    "Hello from Axum!"
}

#[cfg(not(feature = "ssr"))]
fn main() {
    panic!("Server must be built with --features ssr");
}
