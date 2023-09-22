mod clicker;
mod hello;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/hello", get(hello::hello_world))
        .route("/click", get(clicker::click_count))
        .route("/click/ws", get(clicker::click_sock))
}
