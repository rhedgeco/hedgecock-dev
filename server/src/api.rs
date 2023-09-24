mod admin;

use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/admin", admin::router())
}
