use anyhow::Result;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers_core::k256::ecdsa::SigningKey;
use ethers_signers::Wallet;

use crate::dex_utils::config::DeploymentConfig;
use crate::dex_utils::signer::wallet_with_chain_id;
use crate::utils::get_private_key;

pub fn local_signer(
    deployment_config: &DeploymentConfig,
) -> Result<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let provider = Provider::<Http>::try_from(deployment_config.node_url.clone())?;
    let private_key = get_private_key()?;
    let wallet = wallet_with_chain_id(private_key.as_str(), deployment_config.chain_id)?;
    Ok(SignerMiddleware::new(provider.clone(), wallet.clone()))
}
