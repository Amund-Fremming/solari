use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind 0.0.0.0:3000");

    println!("axum-test listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("server failed");
}

async fn health() -> &'static str {
    "healthy"
}
