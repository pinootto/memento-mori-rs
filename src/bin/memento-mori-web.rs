use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = Router::new().route("/", get(home));

    let listener = TcpListener::bind("0.0.0.0:4001").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn home() -> &'static str {
    "memento mori\n"
}
