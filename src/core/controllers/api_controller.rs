use axum::{routing::get, Router};

use crate::AppState;

async fn hello() -> String {
    return String::from("Hello World!");
}

async fn ping() -> String {
    return String::from("pong");
}

pub fn api_router() -> Router<AppState> {
    return Router::new()
        .route("/hello", get(hello))
        .route("/ping", get(ping))
}