use anyhow::Result;
use log::{info, warn};
use tokio::time::Duration;

use oracle::{client, indexer, BTCUSD_INTERNAL_ID, BTC_USD_ORACLE_ID};

pub async fn run(redis_client: &redis::Client) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(3));

    loop {
        let current_btc_price = match client::get_current_price(BTC_USD_ORACLE_ID).await {
            Ok(price) => price,
            Err(e) => {
                warn!("failed to get current price: {}", e);
                interval.tick().await;
                continue;
            }
        };

        indexer::set_price(
            &redis_client,
            BTCUSD_INTERNAL_ID,
            current_btc_price.price_float(),
        )
        .await?;

        info!("set current oracle price complete.");
        interval.tick().await;
    }
}
