use std::path::PathBuf;

use axum::body::HttpBody;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router, ServiceExt};
use clap::Parser;
use futures::SinkExt;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

use args::AppArgs;
use shared::logging::init_logging;

use crate::socket::ws_handler;

mod args;
mod socket;

#[tokio::main]
async fn main() {
    let args = AppArgs::parse();

    init_logging(args.level_filter());

    let mut router = Router::new();
    let ref asset = args.static_content;
    info!("serving static content from {asset:?}");
    if let Ok(dir) = std::fs::read_dir(&asset) {
        let mut root_index_html: Option<PathBuf> = dir
            .into_iter()
            .filter_map(|child| child.ok())
            .find(|entry| entry.file_name() == "index.html")
            .map(|index| index.path());
        match root_index_html {
            None => {
                router = router
                    .fallback_service(ServeDir::new(asset).append_index_html_on_directories(true));
            }
            Some(path) => {
                router = router.fallback_service(
                    ServeDir::new(asset)
                        .append_index_html_on_directories(true)
                        .fallback(ServeFile::new(path)),
                );
            }
        }
    }

    let router = router
        .route("/ws", get(ws_handler))
        .route("/random_uuid", get(generate_uuid))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

async fn generate_uuid() -> impl IntoResponse {
    Json(Uuid::new_v4())
}
