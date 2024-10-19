#[cfg(test)]
mod tests {
    use super::*;

    use crate::dex_utils::bindings;
    use crate::dex_utils::misc::default_subaccount;
    use ethers::abi::AbiEncode;
    use ethers::prelude::transaction::eip712::Eip712;
    use ethers::utils::keccak256;

    #[tokio::test]
    async fn test_call_submit_transactions() -> Result<()> {
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        println!("subaccount: {:02x?}", subaccount);
        let order = bindings::i_offchain_orderbook::Order {
            sender: subaccount,
            price_x18: 0,
            amount: 0,
            expiration: 0,
            nonce: 0,
        };
        let encode = order.clone().encode();
        println!("encode: {:02x?}", encode);

        let struct_hash = keccak256(encode);
        println!("struct_hash: {:02x?}", struct_hash);

        let struct_hashh = order.clone().struct_hash()?;
        println!("struct_hashh: {:02x?}", struct_hashh);

        let domain_separator = order.clone().domain_separator()?;
        println!("domain_separator: {:02x?}", domain_separator);

        let digest_input =
            keccak256([&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat());
        let a = order.clone();
        let digest = a.encode_eip712().unwrap();
        println!("digest: {:02x?}", digest);
        println!("digest_input: {:02x?}", digest_input);

        // let deployment_config = DeploymentConfig::new(ConfigMode::Local)?;
        // let contact_address: Address = "0x67ea3Ea151E1cDF68A9B951C3df1190E2861E75d".parse()?;
        // let provider = Provider::<Http>::try_from(deployment_config.node_url)?;
        // let private_key = get_private_key()?;
        // let wallet = wallet_with_chain_id(private_key.as_str(), deployment_config.chain_id)?;
        // let signer_provider = SignerMiddleware::new(provider.clone(), wallet.clone());
        // let client = Arc::new(signer_provider);
        //
        // let endpoint_contract = bindings::i_offchain_orderbook::IOffchainOrderbook::new(contact_address, client.clone());
        // let digest = endpoint_contract.get_digest(order).call().await?;
        // let domain_separator = order.clone().domain_separator()?;
        // println!("domain_separator: {:02x?}", domain_separator);

        //  [143, 232, 196, 178, 78, 165, 27, 50, 38, 118, 88, 254, 18, 15, 12, 137, 247, 45, 102, 110, 94, 206, 74, 157, 97, 16, 114, 29, 216, 125, 183, 118]
        // println!("digest: {:?}", digest);

        Ok(())
    }
}
