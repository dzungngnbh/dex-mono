use anyhow::Result;
use log::{debug, info, warn};
use tokio::time::Duration;

use oracle::{client, indexer, BTCUSD_INTERNAL_ID, BTC_USD_ORACLE_ID};

// Time interval in seconds between price updates
const UPDATE_INTERVAL: u64 = 3;

/// Runs the BTC price oracle service that periodically updates prices
/// from external source to Redis
pub async fn run(redis_client: &redis::Client) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(UPDATE_INTERVAL));

    // Continuous loop to fetch and update BTC price
    loop {
        if let Err(e) = update_btc_price(redis_client).await {
            warn!("failed to update BTC price: {}", e);
        }
        interval.tick().await;
    }
}

/// Fetches current BTC price from oracle and updates it in Redis
/// Returns Result indicating success or failure of the operation
async fn update_btc_price(redis_client: &redis::Client) -> Result<()> {
    // Get current price from external oracle
    let current_btc_price = client::get_current_price(BTC_USD_ORACLE_ID).await?;

    // Store the price in Redis using internal ID
    indexer::set_price(
        redis_client,
        BTCUSD_INTERNAL_ID,
        current_btc_price.price_float(),
    )
    .await?;

    debug!("set current oracle price complete.");
    Ok(())
}
