use axum::{Router, routing::get};
use sync_wrapper::SyncWrapper;
use std::path::PathBuf;
use axum_extra::routing::SpaRouter;

#[shuttle_service::main]
async fn axum(#[shuttle_static_folder::StaticFolder(folder = "assets")] public_folder: PathBuf)
             -> shuttle_service::ShuttleAxum {

    // initialise the router
    // add a health check route to make sure the api works
    let router = Router::new()
    .route("/api/health", get(health_check))
    .merge(SpaRouter::new("/", &public_folder).index_file("index.html"));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

// async fn health_check() -> &'static str {
//     "OK"
// }