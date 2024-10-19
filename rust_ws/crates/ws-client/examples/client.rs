use anyhow::Result;
use std::env;
use ws_client::{MessageHandler, WebSocketClient};

struct MyMessageHandler;

impl MessageHandler for MyMessageHandler {
    async fn handle(&self, message: String) {
        // for received message, you can use serde_json to try deserialize it and handle.
        println!("Received: {}", message);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let k = env::var("API_KEY").unwrap();
    let handler = MyMessageHandler;
    let mut client =
        WebSocketClient::new("wss://api-jp.stork.network/prices-v3", k.as_str(), handler)
            .await
            .unwrap();

    // Initialize the connection
    client.init().await.unwrap();

    // Send a message
    let message = "{\"action\":\"subscribe\",\"assets\":[\"BTCUSD\",\"ETHUSD\"]}";
    client.sub(message).await.unwrap();

    // Receive messages
    client.receive().await.unwrap();

    Ok(())
}
