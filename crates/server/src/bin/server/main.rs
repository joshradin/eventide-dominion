use std::collections::VecDeque;
use std::path::PathBuf;

use axum::{Router, ServiceExt};
use axum::body::HttpBody;
use axum::response::IntoResponse;
use axum::routing::get;
use clap::Parser;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

use args::AppArgs;
use shared::logging::init_logging;

use crate::socket::ws_handler;

mod args;
mod socket;

#[tokio::main]
async fn main() {
    let args = AppArgs::parse();

    init_logging(args.level_filter());

    let mut assets = VecDeque::from(args.static_content.to_owned().unwrap_or_default());
    assets.push_front(
        PathBuf::from("../../../../..")
            .canonicalize()
            .expect("no current dir")
            .join("public/"),
    );

    info!("serving static content from {assets:?}");

    let mut router = Router::new();

    for asset in assets {
        router =
            router.fallback_service(ServeDir::new(asset).append_index_html_on_directories(true))
    }

    let router = router
        .route("/ws", get(ws_handler))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

async fn index() {}
