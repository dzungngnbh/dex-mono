use crate::dex_utils::bindings::i_endpoint::DepositCollateral;
use ethers::abi::AbiEncode;
use ethers::types::Bytes;
use serde::{Deserialize, Serialize};

// these txs are the one will be submitted by the sequencer

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TxType {
    DepositCollateral = 0,
}

impl DepositCollateral {}

pub fn create_deposit_collateral_tx(deposit_collateral: &DepositCollateral) -> Bytes {
    let mut res = Vec::new();

    let typ: u8 = TxType::DepositCollateral as u8;
    let encoded = deposit_collateral.clone().encode();
    // concat to vec<u8>
    res.push(typ);
    res.extend_from_slice(&encoded);
    Bytes::from(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex_utils::math::ONE_X18;

    use crate::dex_utils::misc::default_subaccount;

    #[test]
    fn test_create_deposit_collateral_tx() -> Result<()> {
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        let deposit_collateral = DepositCollateral {
            subaccount,
            product_id: 3,
            amount: 10 * ONE_X18,
        };

        let tx = create_deposit_collateral_tx(&deposit_collateral);

        println!("tx: {:?}", tx);

        Ok(())
    }
}
