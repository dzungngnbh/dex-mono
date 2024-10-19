use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use shared::db::redis as rediss;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_address: String,
    pub sub_account_balance: Vec<SubAccountBalance>,
    /// in quote value: USDT, DAI
    pub total_balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubAccountBalance {
    pub sub_account: String,
    // must be within 12 bytes
    pub product_balance: Vec<ProductBalance>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductBalance {
    pub product_id: u32,
    pub balance: f64,
}

impl AccountBalance {
    pub fn subaccount_total_balance(&self, sub_account: &str) -> f64 {
        let sub_account_balance = self
            .sub_account_balance
            .iter()
            .find(|s| s.sub_account == sub_account);
        match sub_account_balance {
            Some(sub_account_balance) => sub_account_balance
                .product_balance
                .iter()
                .fold(0.0, |acc, p| acc + p.balance),
            None => 0.0,
        }
    }
}

// index account balance
pub async fn get_balances(redis_client: &redis::Client, sender: &str) -> Result<AccountBalance> {
    let k = account_balance_key(sender);
    match rediss::get(redis_client, &k).await {
        Ok(balances) => Ok(balances),
        Err(_) => {
            bail!("account balance not found");
        }
    }
}

pub async fn set_balance(
    redis_client: &redis::Client,
    sender: &str,
    sub_account: &str,
    product_id: u32,
    balance: f64,
) -> Result<()> {
    let k = account_balance_key(sender);
    let mut account_balance_res: AccountBalance;
    match rediss::get(redis_client, &k).await {
        Ok(balances) => {
            let mut account_balance: AccountBalance = balances;
            match account_balance
                .sub_account_balance
                .iter_mut()
                .find(|s| s.sub_account == sub_account)
            {
                Some(sub_account_balance) => {
                    match sub_account_balance
                        .product_balance
                        .iter_mut()
                        .find(|p| p.product_id == product_id)
                    {
                        Some(product_balance) => {
                            product_balance.balance = balance;
                            let delta_amount = (balance - product_balance.balance).abs();

                            // get
                            let delta =
                                get_delta_balance(redis_client, delta_amount, product_id).await?;

                            account_balance.total_balance = if balance > product_balance.balance {
                                account_balance.total_balance + delta
                            } else {
                                account_balance.total_balance - delta
                            };
                            account_balance_res = account_balance;
                        }
                        None => {
                            sub_account_balance.product_balance.push(ProductBalance {
                                product_id,
                                balance,
                            });
                            let delta =
                                get_delta_balance(redis_client, balance, product_id).await?;
                            account_balance.total_balance += delta;
                            account_balance_res = account_balance;
                        }
                    }
                }
                None => {
                    account_balance.sub_account_balance.push(SubAccountBalance {
                        sub_account: sub_account.to_string(),
                        product_balance: vec![ProductBalance {
                            product_id,
                            balance,
                        }],
                    });
                    let delta = get_delta_balance(redis_client, balance, product_id).await?;
                    account_balance.total_balance += delta;
                    account_balance_res = account_balance;
                }
            }
        }
        Err(_) => {
            let mut account_balance = AccountBalance {
                account_address: sender.to_string(),
                sub_account_balance: vec![],
                total_balance: 0.0,
            };
            account_balance.sub_account_balance.push(SubAccountBalance {
                sub_account: sub_account.to_string(),
                product_balance: vec![ProductBalance {
                    product_id,
                    balance,
                }],
            });
            let delta = get_delta_balance(redis_client, balance, product_id).await?;
            account_balance.total_balance += delta;
            account_balance_res = account_balance;
        }
    }

    // set back to redis
    rediss::set(redis_client, &k, &bincode::serialize(&account_balance_res)?).await?;
    Ok(())
}

async fn get_delta_balance(
    redis_client: &redis::Client,
    delta_amount: f64,
    product_id: u32,
) -> Result<f64> {
    let price = oracle::indexer::get_price(redis_client, product_id).await?;
    Ok(price * delta_amount)
}

fn account_balance_key(sender: &str) -> String {
    format!("account_balances:{}", sender)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::db::redis::get_redis_client;

    async fn reset_balance(redis_client: &redis::Client, sender: &str) -> Result<()> {
        let k = account_balance_key(sender);
        let account_balance = AccountBalance {
            account_address: sender.to_string(),
            sub_account_balance: vec![],
            total_balance: 0.0,
        };
        rediss::set(redis_client, &k, &bincode::serialize(&account_balance)?).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_set_balance() -> Result<()> {
        let redis_client = get_redis_client()?;
        let sender = "0x389Bd6EF1a2E399326Fa6B4AcCEB22D411b8b224";

        set_balance(&redis_client, sender, "default", 3, 0.0).await?;
        set_balance(&redis_client, sender, "default", 0, 100.0).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_balance() -> Result<()> {
        let redis_client = get_redis_client()?;
        let sender = "0x389Bd6EF1a2E399326Fa6B4AcCEB22D411b8b224";
        let account_balances = get_balances(&redis_client, sender).await?;
        dbg!(account_balances);

        Ok(())
    }

    #[tokio::test]
    async fn test_reset_balance() -> Result<()> {
        reset_balance(
            &get_redis_client()?,
            "0x389Bd6EF1a2E399326Fa6B4AcCEB22D411b8b224",
        )
        .await?;
        Ok(())
    }
}
