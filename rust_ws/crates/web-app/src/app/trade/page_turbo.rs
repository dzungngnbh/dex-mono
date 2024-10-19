use crate::app::components::meta::TitleUi;
use anyhow::Result;
use minify_html::{minify, Cfg};

use crate::app::trade::account_stats::TopAccountStats;
use crate::app::trade::TradeStatsUi;
use crate::auth::session_context::SessionContext;

pub struct PageTurbo {}

impl PageTurbo {
    /// Render page turbo
    pub async fn new(
        redis_client: &redis::Client,
        session_context: &SessionContext,
    ) -> Result<String> {
        let mut res = String::new();

        match TradeStatsUi::new(redis_client).await {
            Ok(trade_stats) => {
                // add title
                let title = format!(
                    "{:.2} - {}",
                    trade_stats.oracle_price, trade_stats.product_str
                );
                let title_ui = TitleUi { title };
                match title_ui.render_turbo() {
                    Ok(turbo) => {
                        res.push_str(turbo.as_str());
                    }
                    Err(_err) => {}
                };

                match trade_stats.render_turbo() {
                    Ok(turbo) => {
                        res.push_str(turbo.as_str());
                    }
                    Err(_err) => {}
                };
            }
            Err(_err) => {}
        }

        // TODO: Move to session
        if session_context.account_address.is_some() {
            let sender = session_context.account_address.as_ref().unwrap();
            let sub_account = "default"; // get from frontend
            let product_id = 3; // should get from ws?BTC_DAI convert to productId

            match TopAccountStats::new(redis_client, sender, sub_account, product_id).await {
                Ok(account_stats) => {
                    match account_stats.render_turbo() {
                        Ok(turbo) => {
                            res.push_str(turbo.as_str());
                        }
                        Err(_err) => {}
                    };
                }
                Err(_err) => {}
            }
        }

        let res = minify(res.as_bytes(), &Cfg::default());
        Ok(String::from_utf8(res)?)
    }
}
