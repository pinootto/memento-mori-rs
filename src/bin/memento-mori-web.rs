use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use jiff::civil::Date;
use memento_mori_rs::{build_calendar, Args, TimeUnit};
use serde::Deserialize;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*};

#[derive(Debug, Deserialize)]
struct QueryParams {
    birthday: Date,
    death_age: Option<u8>,
    time_unit: Option<TimeUnit>,
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry().with(fmt::layer()).init();

    let router = Router::new()
        .route("/", get(home))
        .route("/calendar", get(show_calendar));

    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn home() -> &'static str {
    "memento mori\n"
}

async fn show_calendar(Query(params): Query<QueryParams>) -> impl IntoResponse {
    let death_age = params.death_age.unwrap_or(90);
    let time_unit = params.time_unit.unwrap_or(TimeUnit::Month);
    println!("{} {} {}", params.birthday, death_age, time_unit);
    let args = Args::new(params.birthday, death_age, time_unit);
    build_calendar(args)
}
