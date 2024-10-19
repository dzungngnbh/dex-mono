use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::Address;
use ethers_signers::{Signer, Wallet};

use crate::dex_utils::config::{ConfigMode, DeploymentConfig};
use crate::dex_utils::signer::wallet_with_chain_id;

#[derive(Clone)]
pub struct DexClient {
    pub deployment_config: DeploymentConfig,
    pub wallet: Option<Wallet<SigningKey>>,
}

impl DexClient {
    pub fn new(config_mode: ConfigMode, private_key: String) -> Result<Self> {
        let deployment_config = DeploymentConfig::new(config_mode)?;

        let wallet =
            wallet_with_chain_id(private_key.as_str(), deployment_config.chain_id).unwrap();

        Ok(DexClient {
            deployment_config,
            wallet: Some(wallet),
        })
    }

    pub fn address(&self) -> Result<Address> {
        Ok(self.wallet.as_ref().unwrap().address())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex_utils::bindings::i_endpoint;
    use crate::dex_utils::types::AddressBytes;
    use crate::utils::get_private_key;
    use ethers::utils::hex;

    #[test]
    fn test_dex_client_new() -> Result<()> {
        let private_key = get_private_key()?;
        let dex_client = DexClient::new(ConfigMode::Local, private_key)?;

        let address = dex_client.wallet.as_ref().unwrap().address();
        let address_bytes: AddressBytes = address.into();
        println!("address: {:?}", address);
        println!("address_bytes: {:?}", address_bytes);

        Ok(())
    }

    #[tokio::test]
    async fn test_call_submit_transactions() {
        let first: u8 = 0;
    }
}
