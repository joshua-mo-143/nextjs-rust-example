use axum::{Router};
use sync_wrapper::SyncWrapper;
use std::path::PathBuf;
use axum_extra::routing::SpaRouter;

// a data structure for designing what the app state should look like
struct AppState {
    public: PathBuf
}

#[shuttle_service::main]
async fn axum(#[shuttle_static_folder::StaticFolder(folder = "out")] public_folder: PathBuf)
             -> shuttle_service::ShuttleAxum {

    // initialise an AppState struct that can be used across any route
    // This is basically Rust's equivalent of global state in React/next etc
    let app = AppState{public: public_folder};

    // initialise the router using the spa router function
    let router = spa_router(&app);
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

fn spa_router(app: &AppState) -> Router {
    // `SpaRouter` is the easiest way to serve assets at a nested route like `/assets`
    //
    // Requests starting with `/assets` will be served from files in the current directory.
    // Requests to unknown routes will get `index.html`.
    Router::new()
        .merge(SpaRouter::new("/", &app.public).index_file("index.html"))
}
