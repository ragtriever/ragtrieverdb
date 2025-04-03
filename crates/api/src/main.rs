use axum::routing::{Router, get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/health", get(|| async { "OK" }))
        .route("/api/v1/metrics", get(|| async { "Metrics" }))
        .route("/api/v1/healthz", get(|| async { "Health Check" }))
        .route("/api/v1/readyz", get(|| async { "Readiness Check" }))
        .route("/api/v1/livez", get(|| async { "Liveness Check" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
