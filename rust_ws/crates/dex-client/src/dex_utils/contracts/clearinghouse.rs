use anyhow::Result;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{Http, Provider};
use ethers::types::Address;
use ethers_signers::Wallet;
use std::sync::Arc;

use crate::dex_utils::bindings::i_clearinghouse;
use crate::dex_utils::config::{ConfigMode, DeploymentConfig};
use crate::dex_utils::contracts::utils::local_signer;

pub struct Clearinghouse {
    pub contract:
        i_clearinghouse::IClearinghouse<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Clearinghouse {
    pub fn new(
        contract_address: Address,
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Self {
        let contract = i_clearinghouse::IClearinghouse::new(contract_address, client.clone());
        Self { contract }
    }

    pub async fn get_health(&self, sub_account: &[u8; 32], health_type: u8) -> Result<i128> {
        Ok(self
            .contract
            .get_health(*sub_account, health_type)
            .call()
            .await?)
    }
}

pub fn get_test_clearinghouse() -> Result<Clearinghouse> {
    let deployment_config = DeploymentConfig::new(ConfigMode::Local)?;
    let contract_address: Address = deployment_config.clearinghouse.parse()?;
    let client = Arc::new(local_signer(&deployment_config)?);
    Ok(Clearinghouse::new(contract_address, client))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_health() -> Result<()> {
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        let health_type: u8 = 0; // which is initial

        let clearinghouse = get_test_clearinghouse()?;
        let initHealth = clearinghouse.get_health(&subaccount, health_type).await?;
        println!("initHealth: {}", initHealth);
        Ok(())
    }
}
