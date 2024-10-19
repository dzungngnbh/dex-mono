use anyhow::{bail, Result};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{
    connect_async_with_config, tungstenite::protocol::Message, MaybeTlsStream,
};
use url::Url;

pub trait MessageHandler {
    async fn handle(&self, message: String);
}

pub struct WebSocketClient<H: MessageHandler + Send + Sync> {
    ws_stream: tokio_tungstenite::WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    message_handler: H,
}

impl<H: MessageHandler + Send + Sync> WebSocketClient<H> {
    pub async fn new(url: &str, auth_token: &str, handler: H) -> Result<Self> {
        let url = Url::parse(url)?;
        let mut req = url.into_client_request()?;
        // req.headers_mut()
        //     .insert("Authorization", auth_token.parse()?);

        // parse url and create a request

        let (ws_stream, _) = connect_async_with_config(req, None, false).await?;

        Ok(Self {
            ws_stream,
            message_handler: handler,
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        // Initialization logic goes here
        Ok(())
    }

    pub async fn sub(&mut self, message: &str) -> Result<()> {
        self.ws_stream
            .send(Message::Text(message.to_string()))
            .await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<()> {
        while let Some(result) = self.ws_stream.next().await {
            match result {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        self.message_handler.handle(text).await;
                    }
                }
                Err(e) => bail!(e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
