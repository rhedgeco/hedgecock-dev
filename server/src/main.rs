use std::{path::PathBuf, sync::Arc};

use axum::{Extension, Router};
use hedgecock_dev::api;
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use shuttle_secrets::SecretStore;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // create static folder and random assets
    let static_folder = PathBuf::from("./static");
    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

    // create router and add layers
    let router = router(&static_folder)
        .layer(Extension(secrets))
        .layer(Extension(Arc::new(Mutex::new(random))));

    // return the router
    Ok(router.into())
}

fn router(static_folder: &PathBuf) -> Router {
    Router::new().nest("/api", api::router()).fallback_service(
        ServeDir::new(static_folder)
            .not_found_service(ServeFile::new(static_folder.join("index.html"))),
    )
}
