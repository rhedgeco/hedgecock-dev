use std::path::PathBuf;

use astro_blaster::api;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let static_folder = PathBuf::from("./static");
    let router = router(&static_folder);

    Ok(router.into())
}

fn router(static_folder: &PathBuf) -> Router {
    Router::new().nest("/api", api::router()).fallback_service(
        ServeDir::new(static_folder)
            .not_found_service(ServeFile::new(static_folder.join("index.html"))),
    )
}
