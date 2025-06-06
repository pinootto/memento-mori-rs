use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use jiff::civil::Date;
use memento_mori_rs::{build_calendar, Args, TimeUnit};
use serde::Deserialize;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::{
    fmt::{self, format},
    prelude::*,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct WebArgs {
    #[arg(short, long)]
    #[arg(default_value_t = 4001)]
    #[arg(value_parser = clap::value_parser!(u16).range(0..=65535))]
    port: u16,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    birthday: Date,
    death_age: Option<u8>,
    time_unit: Option<TimeUnit>,
}

#[tokio::main]
async fn main() {
    // set log level with env variable RUST_LOG
    tracing_subscriber::fmt::init();
    // print all log levels
    // tracing_subscriber::registry().with(fmt::layer()).init();

    let web_args = WebArgs::parse();

    let router = Router::new()
        .route("/", get(home))
        .route("/calendar", get(show_calendar));

    let address_port = format!("0.0.0.0:{}", web_args.port);
    let listener = TcpListener::bind(address_port).await.unwrap();
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
