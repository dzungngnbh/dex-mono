use ethers::middleware::SignerMiddleware;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{Http, Provider};
use ethers::types::Address;
use ethers_signers::Wallet;
use std::sync::Arc;

use crate::dex_utils::bindings::i_endpoint;

pub struct Endpoint {
    pub contract: i_endpoint::IEndpoint<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Endpoint {
    pub fn new(
        contract_address: Address,
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    ) -> Self {
        let contract = i_endpoint::IEndpoint::new(contract_address, client.clone());
        Self { contract }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex_utils::bindings::i_endpoint;
    use crate::dex_utils::contracts::utils::local_signer;

    #[tokio::test]
    async fn test_call_submit_transactions() -> Result<()> {
        let subaccount = default_subaccount("0x389Bd6EF1a2E399326Fa6B4AcCEB22D411b8b224")?;
        let deposit_collateral = DepositCollateral {
            subaccount,
            product_id: 0,
            amount: 0,
        };

        let tx = crate::dex_utils::tx::create_deposit_collateral_tx(&deposit_collateral);
        let txs = vec![tx];

        let deployment_config = DeploymentConfig::new(ConfigMode::Local)?;
        let contract_address: Address = deployment_config.endpoint.parse()?;
        let client = Arc::new(local_signer(&deployment_config)?);

        let endpoint_contract = i_endpoint::IEndpoint::new(contract_address, client.clone());
        endpoint_contract.submit_transactions(txs).call().await?;

        Ok(())
    }
}
