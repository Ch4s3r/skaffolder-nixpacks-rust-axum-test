use axum::{
    response::Html,
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("🚀 Server running on http://0.0.0.0:8080");
    println!("📡 Endpoints:");
    println!("   GET / - Hello World");
    println!("   GET /health - Health Check");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World! 🦀</h1><p>Welcome to your Rust Axum server running in Kubernetes!</p>")
}

async fn health_check() -> &'static str {
    "OK"
}
