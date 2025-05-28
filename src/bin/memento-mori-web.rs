use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use jiff::civil::Date;
use memento_mori_rs::TimeUnit;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct QueryParams {
    birthday: Date,
    death_age: u8,
    time_unit: TimeUnit,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let router = Router::new()
        .route("/", get(home))
        .route("/by_week", get(show_by_week));

    let listener = TcpListener::bind("0.0.0.0:4001").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn home() -> &'static str {
    "memento mori\n"
}

async fn show_by_week(Query(params): Query<QueryParams>) -> impl IntoResponse {
    format!(
        "{} {} {}",
        params.birthday, params.death_age, params.time_unit
    )
}
