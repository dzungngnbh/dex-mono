use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MarketHours {
    pub is_open: bool,
    pub next_open: Option<i64>,
    pub next_close: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct Attributes {
    pub symbol: String,
    pub asset_type: String,
    pub base: String,
    pub cms_symbol: Option<String>,
    pub country: Option<String>,
    pub cqs_symbol: Option<String>,
    pub description: String,
    pub generic_symbol: Option<String>,
    pub nasdaq_symbol: Option<String>,
    pub quote_currency: String,
    pub weekly_schedule: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceFeedResponse {
    pub id: String,
    pub market_hours: MarketHours,
    pub attributes: Attributes,
}

// PriceUpdateResponse
#[derive(Debug, Deserialize)]
pub struct PriceUpdateResp {
    pub parsed: Vec<PriceUpdateData>,
    // ignore the binary
}

#[derive(Debug, Deserialize)]
struct BinaryData {
    encoding: String,

    #[serde(skip_deserializing)]
    data: String,
}

#[derive(Debug, Deserialize)]
struct PriceUpdateData {
    pub id: String,
    pub price: PriceData,
    pub ema_price: PriceData,
}

#[derive(Debug, Deserialize)]
pub struct PriceData {
    pub conf: String,
    pub expo: i8,
    pub price: String,
    #[serde(skip_deserializing)]
    pub price_float: f64,
    pub publish_time: i64,
}

impl PriceUpdateResp {
    pub fn price_float(&self) -> f64 {
        self.parsed.first().unwrap().price.price_float
    }
}

// https://pyth.network/developers/price-feed-ids
pub async fn get_price_feed(price_feed_id: &str) -> Result<PriceFeedResponse> {
    let url = format!("https://benchmarks.pyth.network/v1/price_feeds/{price_feed_id}");
    let resp = reqwest::get(url).await?;
    // convert to PriceFeedResponse
    let price_feed: PriceFeedResponse = resp.json().await?;
    Ok(price_feed)
}

pub async fn get_current_price(price_feed_id: &str) -> Result<PriceUpdateResp> {
    let ts = time::OffsetDateTime::now_utc().unix_timestamp() - 20; // 20 seconds ago
    let url = format!("https://benchmarks.pyth.network/v1/updates/price/{ts}?ids={price_feed_id}&encoding=hex&parsed=true");
    let mut price_update_resp: PriceUpdateResp = reqwest::get(url).await?.json().await?;

    if !price_update_resp.parsed.is_empty() {
        let price_update_data = price_update_resp.parsed.first_mut().unwrap();
        price_update_data.price.price_float =
            price_to_float(&price_update_data.price.price, price_update_data.price.expo)?;
    }
    Ok(price_update_resp)
}

fn price_to_float(price: &str, expo: i8) -> Result<f64> {
    // add dot to price at at expo character from the end
    let mut price = price.to_string();
    let len = price.len();
    let dot_index = len - expo.unsigned_abs() as usize;
    price.insert(dot_index, '.');
    price
        .parse::<f64>()
        .map_err(|e| anyhow::anyhow!("price_to_float: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::BTC_USD_ORACLE_ID;

    #[tokio::test]
    async fn test_get_price_feed() -> Result<()> {
        let price_feed = get_price_feed(BTC_USD_ORACLE_ID).await?;
        dbg!(price_feed);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_current_price() -> Result<()> {
        let price_update_resp = get_current_price(BTC_USD_ORACLE_ID).await?;
        dbg!(price_update_resp);
        Ok(())
    }
}
