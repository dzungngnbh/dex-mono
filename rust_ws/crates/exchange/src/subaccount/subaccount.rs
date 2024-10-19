use anyhow::Result;
use dex_client::dex_utils::contracts::clearinghouse::Clearinghouse;

use dex_client::dex_utils::contracts::spot_engine::{QuoteBaseBalances, SpotEngine};
use dex_client::dex_utils::types::SubAccountBytes;

use crate::orderbook::order::Order;
use crate::orderbook::orderbook::{get_orderbook, OrderBook};

pub struct SubAccount<'a> {
    pub sub_account: SubAccountBytes,
    spot_engine: &'a SpotEngine,
    clearinghouse: &'a Clearinghouse,
}

pub struct ProductBalance {
    pub base: f64,
    pub quote: f64,
}

// TODO: For SpotEngine and Clearinghouse, we just need public one, no need signing one, since
// the signing part happening on browser, and only client for dex_client.
impl<'a> SubAccount<'a> {
    pub fn new(
        sub_account: SubAccountBytes,
        spot_engine: &'a SpotEngine,
        clearinghouse: &'a Clearinghouse,
    ) -> Self {
        Self {
            sub_account,
            spot_engine,
            clearinghouse,
        }
    }

    /// Order must be check valid before calling this function
    pub async fn can_place_order(&self, order: &Order) -> Result<bool> {
        // get health make sure health is greater than 0
        let init_health = self.get_health(0).await?;
        if init_health <= 0 {
            return Ok(false);
        }

        let product_balances = self.get_product_balances(order.product_id).await?;
        if order.amount > 0.0 && product_balances.base == 0.0 {
            // cannot buy if you dont have base
            return Ok(false);
        }

        if order.amount < 0.0 && product_balances.quote == 0.0 {
            // cannot sell if you dont have quote
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn place_order(&self, order: &Order) -> Result<()> {
        // send to orderbook to getmatched
        let orderbook = OrderBook::get(order.product_id)?;

        Ok(())
    }

    /// get health on chain, to see if it can make order
    pub async fn get_health(&self, health_type: u8) -> Result<i128> {
        Ok(self
            .clearinghouse
            .get_health(&self.sub_account, health_type)
            .await?)
    }

    /// returns balance of the subaccount with base and quote
    pub async fn get_product_balances(&self, product_id: u32) -> Result<QuoteBaseBalances> {
        Ok(self
            .spot_engine
            .get_quote_base_balances(self.sub_account, product_id)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dex_client::dex_utils::contracts::clearinghouse::get_test_clearinghouse;
    use dex_client::dex_utils::contracts::spot_engine::get_test_spot_engine;
    use dex_client::dex_utils::contracts::BTC_DAI_PRODUCT_ID;
    use dex_client::dex_utils::misc::default_subaccount;

    use crate::orderbook::order_command::OrderType;

    #[tokio::test]
    async fn test_can_place_order() -> Result<()> {
        dotenvy::dotenv().ok();
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        // TODO: These 2 use the same signer, we can refactor later
        let spot_engine = get_test_spot_engine()?;
        let clearinghouse = get_test_clearinghouse()?;

        let order = Order {
            amount: 1.0,
            price: 1.0,
            order_type: OrderType::GTC,
            product_id: BTC_DAI_PRODUCT_ID,
            order_digest: [1u8; 32], // fake to avoid invalid
            timestamp: 0,
        };
        assert!(order.is_valid());

        let sub_account = SubAccount::new(subaccount, &spot_engine, &clearinghouse);
        let can_place = sub_account.can_place_order(&order).await?;

        assert!(can_place);
        Ok(())
    }
}
