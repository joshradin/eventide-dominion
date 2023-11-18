use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{headers, TypedHeader};
use std::net::SocketAddr;
use tracing::{info, instrument};

/// Web socket handler
#[instrument]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {}
