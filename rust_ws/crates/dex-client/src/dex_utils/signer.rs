use std::str::FromStr;

use anyhow::Result;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers_signers::{Signer, Wallet};

pub fn wallet_with_chain_id(private_key: &str, chain_id: u64) -> Result<Wallet<SigningKey>> {
    let wallet = Wallet::from_str(private_key)?;
    Ok(wallet.with_chain_id(chain_id))
}
