use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::Response;
use axum::Extension;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use log::{info, error};
use serde_json::json;

use crate::api::auth::session_context::SessionContext;
use crate::backend::Backend;

pub async fn traders_ws_handler(
    Extension(backend): Extension<Backend>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    session_context: SessionContext,
) -> Response {
    info!("traders websocket started");
    let backend_arc = Arc::new(backend);
    let session_arc = Arc::new(session_context);
    ws.on_upgrade(move |socket| async move {
        match handle_socket(backend_arc, session_arc, socket, addr).await {
            Ok(()) => {}
            Err(e) => {
                error!("{} disconnected due to error: {}", addr, e);
            }
        }
    })
}

async fn handle_socket(
    backend: Arc<Backend>,
    session_context: Arc<SessionContext>,
    socket: WebSocket,
    who: SocketAddr,
) -> Result<()> {
    // Split socket for concurrent send and receive
    let (mut sender, receiver) = socket.split();

    // Spawn a task that will push several messages to the client
    let send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
        loop {
            let trader_data = json!({
                "account": "SHIBA",
                "youtubeUrl": "127 days",
                "lastUpload": "2 days ago",
                "market": "DEX",
                "price": "$1,433,438",
                "totalVolume": "$5,333,935",
                "avgVolume": "$205,039",
                "change": "21.78%",
                "trades": "23",
                "tradesC": "14",
                "avgPrice": "77.78%",
                "avgPriceC": "91.12%",
                "adCoverage": "85%",
                "adCoverageC": "92%",
                "adDuration": "35s",
                "adDurationC": "-14.36%"
            });

            if sender.send(Message::Text(trader_data.to_string())).await.is_err() {
                break;
            }

            interval.tick().await;
        }
    });

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
    Ok(())
}
