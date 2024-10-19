use anyhow::Result;
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::Response;
use axum::Extension;
use futures::{SinkExt, StreamExt};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::app::trade;
use crate::auth::session_context::SessionContext;
use crate::backend::Backend;

pub async fn ws_handler(
    Extension(backend): Extension<Backend>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    session_context: SessionContext,
) -> Response {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    let backend_arc = Arc::new(backend);
    let session_arc = Arc::new(session_context);
    ws.on_upgrade(move |socket| async move {
        match handle_socket(backend_arc, session_arc, socket, addr).await {
            Ok(()) => {}
            Err(e) => {
                println!("{} disconnected due to error: {}", addr, e);
            }
        }
    })
}

async fn handle_socket(
    backend: Arc<Backend>,
    session_context: Arc<SessionContext>,
    mut socket: WebSocket,
    who: SocketAddr,
) -> Result<()> {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return Ok(());
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
        loop {
            let page_turbo = trade::PageTurbo::new(&backend.redis_client, &session_context)
                .await
                .unwrap();

            // if we have sender info, we send their balances to frontend.
            if sender.send(Message::Text(page_turbo)).await.is_err() {
                break;
            }

            interval.tick().await;
        }

        println!("Sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {e}, probably it is ok?");
        }
    });

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
    Ok(())
}
