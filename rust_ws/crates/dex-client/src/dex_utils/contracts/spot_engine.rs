use anyhow::Result;
use ethers::addressbook::Address;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::Provider;
use ethers::providers::Http;
use ethers_signers::Wallet;
use std::sync::Arc;

use crate::dex_utils::bindings::i_spot_engine::i_spot_engine;
use crate::dex_utils::config::{ConfigMode, DeploymentConfig};
use crate::dex_utils::contracts::utils::local_signer;
use crate::dex_utils::contracts::PRODUCT_DECIMALS;
use crate::dex_utils::types::SubAccountBytes;

/// A wrapper around the SpotEngine binding contract
pub struct SpotEngine {
    pub contract: i_spot_engine::ISpotEngine<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

#[derive(Debug)]
pub struct SubaccountBalance_f64 {
    pub balance: f64,
    pub lp_balance: f64,
}

#[derive(Debug)]
pub struct QuoteBaseBalances {
    pub quote: f64,
    pub quote_id: u32,

    pub base: f64,
    pub base_id: u32,
    // TODO: Handle lp amount later
}

impl SpotEngine {
    pub fn new(
        contract_address: Address,
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Self {
        let contract = i_spot_engine::ISpotEngine::new(contract_address, client.clone());
        Self { contract }
    }

    /// Get the balance of a subaccount for a given product id
    pub async fn get_balance(
        &self,
        subaccount: SubAccountBytes,
        product_id: u32,
    ) -> Result<SubaccountBalance_f64> {
        let bal = self
            .contract
            .get_balance(subaccount, product_id)
            .call()
            .await?;
        let decimals = *PRODUCT_DECIMALS.get(&product_id).unwrap();
        let denom = 10u64.pow(decimals) as f64;
        Ok(SubaccountBalance_f64 {
            balance: bal.balance as f64 / denom,
            lp_balance: bal.lp_balance as f64 / denom,
        })
    }

    /// get quote base balance
    pub async fn get_quote_base_balances(
        &self,
        subaccount: SubAccountBytes,
        quote_product_id: u32,
    ) -> Result<QuoteBaseBalances> {
        let quote_balance = self.get_balance(subaccount, quote_product_id).await?;
        let base_balance = self.get_balance(subaccount, 0).await?;
        let quote_base_balances = QuoteBaseBalances {
            quote: quote_balance.balance,
            quote_id: quote_product_id,
            base: base_balance.balance,
            base_id: 0,
        };
        Ok(quote_base_balances)
    }
}

pub fn get_test_spot_engine() -> Result<SpotEngine> {
    let deployment_config = DeploymentConfig::new(ConfigMode::Local)?;
    let contract_address: Address = deployment_config.spot_engine.parse()?;
    let client = Arc::new(local_signer(&deployment_config)?);
    Ok(SpotEngine::new(contract_address, client.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex_utils::misc::default_subaccount;

    #[tokio::test]
    async fn test_get_subaccount_balance() -> Result<()> {
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        let contract = get_test_spot_engine()?;
        let bal = contract.get_balance(subaccount, 3).await?;
        println!("bal: {:?}", bal);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_quote_base_balances() -> Result<()> {
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        let contract = get_test_spot_engine()?;
        let bal = contract.get_quote_base_balances(subaccount, 3).await?;
        dbg!(&bal);

        Ok(())
    }
}
