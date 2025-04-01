use anyhow::Result;
use redis::Client;
use sailfish::TemplateOnce;

use oracle::{BTCUSD_INTERNAL_ID, indexer};

use crate::pages::components::hotwired_turbo::TurboStream;
use crate::pages::components::hotwired_turbo::turbo_stream::ACTION_REPLACE;

pub struct TradeStatsUi {
    pub product_str: String,
    pub quote_price: f64,
    pub daily_change: f64,
    pub daily_volume: f64,
    pub oracle_price: f64,
    pub spot_index_price: f64,
}

#[derive(TemplateOnce, Default)]
#[template(path = "trade/trade_stats.stpl")]
struct TradeStatsTpl<'a> {
    pub quote_price: &'a str,
    pub quote_price_usd: &'a str,
    pub daily_change: &'a str,
    pub daily_volume: &'a str,
    pub oracle_price: &'a str,
    pub spot_index_price: &'a str,
}

impl TradeStatsUi {
    pub async fn new(redis_client: &Client) -> Result<Self> {
        let quote_price_float = indexer::get_price(redis_client, BTCUSD_INTERNAL_ID).await?;

        Ok(Self {
            product_str: "BTC/DAI".to_string(),
            quote_price: quote_price_float,
            daily_change: 0.0,
            daily_volume: 0.0,
            oracle_price: quote_price_float,
            spot_index_price: 0.0,
        })
    }

    pub fn render_tpl(&self) -> Result<String> {
        let quote_price_usd = self.quote_price * 1.01;
        let quote_price_usd_str = format!("${:.2}", quote_price_usd);
        let daily_change_str = if self.daily_change >= 0.0 {
            format!("+{:.2}%", self.daily_change * 100.0)
        } else {
            format!("-{:.2}%", self.daily_change * 100.0)
        };
        let daily_volume_str = format!("${:.2}", self.daily_volume);
        let oracle_price_str = format!("${:.2}", self.oracle_price);
        let spot_index_price_str = format!("${:.2}", self.spot_index_price);
        let quote_price_ui = TradeStatsTpl {
            quote_price: format!("{:.2}", &self.quote_price).as_str(),
            quote_price_usd: quote_price_usd_str.as_str(),
            daily_change: daily_change_str.as_str(),
            daily_volume: daily_volume_str.as_str(),
            oracle_price: oracle_price_str.as_str(),
            spot_index_price: spot_index_price_str.as_str(),
        }
        .render_once()?;

        Ok(quote_price_ui)
    }

    pub fn render_turbo(&self) -> Result<String> {
        let quote_price_ui = self.render_tpl()?;
        let turbo_stream = TurboStream {
            action: ACTION_REPLACE,
            target: "trade-stats",
            template_ui: &quote_price_ui,
        }
        .render_once()?;

        Ok(turbo_stream)
    }
}

mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_render_trade_stats() -> Result<()> {
        let trade_stats_ui = TradeStatsUi {
            product_str: "BTC/DAI".to_string(),
            quote_price: 10_000.00,
            daily_change: 0.1,
            daily_volume: 1_000_000.00,
            oracle_price: 10_001.00,
            spot_index_price: 10_002.00,
        };
        let res = trade_stats_ui.render_tpl()?;

        Ok(())
    }
}
