use crate::app::components::hotwired_turbo::turbo_stream::ACTION_REPLACE;
use crate::app::components::hotwired_turbo::TurboStream;
use anyhow::Result;
use sailfish::TemplateOnce;

/// Total stats of acount
pub struct TopAccountStats {
    /// Total balance of all products id
    pub total_balance: f64,

    /// Current sub account stats
    pub sub_account_stats: SubAccountStats,
}

/// Current account stats
pub struct SubAccountStats {
    pub product_id: u32,
    pub sub_account: String,
    pub total_balance: f64,
}

#[derive(TemplateOnce, Default)]
#[template(path = "trade/top_account_stats.stpl")]
struct TopAccountStatsUi<'a> {
    pub total_balance: &'a str,
    pub sub_account_balance: &'a str,
}

impl TopAccountStats {
    pub async fn new(
        redis_client: &redis::Client,
        sender: &str,
        sub_account: &str,
        product_id: u32,
    ) -> Result<Self> {
        let account_balance = indexer_service::account::get_balances(redis_client, sender).await?;

        Ok(Self {
            total_balance: account_balance.total_balance,
            sub_account_stats: SubAccountStats {
                product_id,
                sub_account: sub_account.to_string(),
                total_balance: account_balance.subaccount_total_balance(sub_account),
            },
        })
    }

    pub fn render_tpl(&self) -> Result<String> {
        let total_balance_str = format!("${:.2}", self.total_balance);
        let sub_account_balance_str = format!("${:.2}", self.sub_account_stats.total_balance);
        let ui_str = TopAccountStatsUi {
            total_balance: total_balance_str.as_str(),
            sub_account_balance: sub_account_balance_str.as_str(),
        }
        .render_once()?;

        Ok(ui_str)
    }

    pub fn render_turbo(&self) -> Result<String> {
        let ui_str = self.render_tpl()?;
        let turbo_stream = TurboStream {
            action: ACTION_REPLACE,
            target: "top-account-stats",
            template_ui: &ui_str,
        };
        Ok(turbo_stream.render_once()?)
    }
}
