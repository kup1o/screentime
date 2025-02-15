mod api;
mod client;
mod last_usage_time;
mod screentime_updater;
mod utils;

use axum::Router;
use chrono::Utc;

use std::collections::HashMap;

use axum::routing::get;

type ScreenTime = HashMap<String, u32>;

fn build_router() -> Router {
    Router::new()
        .route("/inlinehms", get(&api::get_inlinehms))
        .route("/json", get(&api::get_json_secs))
}

#[tokio::main]
async fn main() {
    if std::env::args().nth(1).unwrap_or("missing".into()) != "-d".to_string() {
        client::run_client();
        return;
    }
    let (rx, tx) = single_value_channel::channel_starting_with(Utc::now());
    tokio::task::spawn(screentime_updater::run_screentime_updater(rx));
    tokio::task::spawn(last_usage_time::run_last_usage_time_updater(tx));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8465").await.unwrap();
    axum::serve(listener, build_router()).await.unwrap();
}
