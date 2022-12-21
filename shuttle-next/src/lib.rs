use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum(#[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: PathBuf)
             -> shuttle_service::ShuttleAxum {

    let router = Router::new().route("/", get(public_folder.test));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}