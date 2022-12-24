use axum::{Router, routing::get};
use sync_wrapper::SyncWrapper;
use std::path::PathBuf;
use axum_extra::routing::SpaRouter;

// a data structure for designing what the app state should look like
struct AppState {
    public: PathBuf
}

#[shuttle_service::main]
async fn axum(#[shuttle_static_folder::StaticFolder(folder = "assets")] public_folder: PathBuf)
             -> shuttle_service::ShuttleAxum {

    // initialise an AppState struct that can be used across any route
    // This is basically Rust's equivalent of global state in React/next etc
    let app = AppState{public: public_folder};

    // initialise the router
    // add a health check route to make sure the api works
    let router = Router::new()
    .route("/api/health", get(health_check))
    .merge(SpaRouter::new("/", &app.public).index_file("index.html"));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

async fn health_check() -> &'static str {
    "OK"
}