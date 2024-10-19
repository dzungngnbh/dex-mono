use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::is_quote_product;
use shared::db::redis as rediss;

#[derive(Serialize, Deserialize)]
struct Price {
    price: f64,
}

// override your own function
pub fn price_key(product_id: u32) -> String {
    format!("price_{}", product_id)
}

// customized function to work with f64 only
// rolling your own if you need
pub async fn set_price(client: &redis::Client, product_id: u32, price: f64) -> Result<()> {
    let k = price_key(product_id);
    let price = Price { price };
    rediss::set(client, k.as_str(), &bincode::serialize(&price)?).await?;

    Ok(())
}

pub async fn get_price(client: &redis::Client, product_id: u32) -> Result<f64> {
    if is_quote_product(product_id) {
        return Ok(1.0); // TODO: Deal with this rate later
    }

    let k = price_key(product_id);
    let price: Price = rediss::get(client, k.as_str()).await?;
    Ok(price.price)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_redis_client() -> Result<redis::Client> {
        dotenvy::dotenv().ok();
        rediss::get_redis_client()
    }

    // this is integration test, make sure to have dragonfly/redis running
    #[tokio::test]
    async fn test_set_price() -> Result<()> {
        let client = get_redis_client()?;
        set_price(&client, 1, 100.0).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_current_price() -> Result<()> {
        let client = get_redis_client()?;
        let price = get_price(&client, 1).await?;
        assert_eq!(price, 100.0);

        Ok(())
    }
}
