use anyhow::{bail, Result};
use ethers::abi::AbiEncode;
use ethers::types::transaction::eip712::{EIP712Domain, Eip712};
use ethers::types::{Address, H160, H256, U256};
use ethers::utils::keccak256;
use std::fmt::Debug;
use std::str::FromStr;

use crate::dex_utils::bindings;
use crate::dex_utils::types::{AddressBytes, SubAccountBytes};

// convert string to bytes with fixed size
// https://stackoverflow.com/questions/75570888/copying-a-slice-of-bytes-into-an-array-of-non-matching-size
macro_rules! to_bytes {
    ($s: expr, $sz: expr) => {{
        let mut res = [0u8; $sz];
        let n = std::cmp::min($s.as_bytes().len(), $sz);
        res[..n].copy_from_slice(&$s.as_bytes()[..n]);
        res
    }};
}

pub fn default_subaccount(address: &str) -> Result<SubAccountBytes> {
    let address = Address::from_str(address)?;
    create_subaccount(address, "default")
}

pub fn create_subaccount(address: Address, subaccount: &str) -> Result<SubAccountBytes> {
    if subaccount.len() > 12 {
        return bail!("subaccount name length should be less than 12");
    }

    let address_bytes: AddressBytes = address.into();
    let subaccount: [u8; 12] = to_bytes!(subaccount, 12);
    let mut res = [0u8; 32];
    res[..20].copy_from_slice(&address_bytes);
    res[20..].copy_from_slice(&subaccount);
    Ok(res)
}

pub fn get_eip712_digest<T: Eip712 + Send + Sync + Debug>(
    payload: &T,
    domain: &EIP712Domain,
) -> H256 {
    let domain_separator = domain.separator();
    let struct_hash = payload.struct_hash().unwrap();
    let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();
    H256::from(keccak256(digest_input))
}

// 0x4b5dde172732338a171d63d3b949aacf6bb234046f1395e1986341aad20e7d48
const ORDER_TYPE_HASH: [u8; 32] = [
    0x4b, 0x5d, 0xde, 0x17, 0x27, 0x32, 0x33, 0x8a, 0x17, 0x1d, 0x63, 0xd3, 0xb9, 0x49, 0xaa, 0xcf,
    0x6b, 0xb2, 0x34, 0x04, 0x6f, 0x13, 0x95, 0xe1, 0x98, 0x63, 0x41, 0xaa, 0xd2, 0x0e, 0x7d, 0x48,
];
pub fn get_order_digest(
    payload: &bindings::i_endpoint::Order,
    domain: &EIP712Domain,
) -> Result<H256> {
    let domain_separator = domain.separator();
    let order_hash = payload.clone().encode();
    let struct_hash = [&ORDER_TYPE_HASH[..], &order_hash[..]].concat();
    let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();
    Ok(H256::from(keccak256(digest_input)))
}

pub fn domain(chain_id: U256, verifying_contract: H160) -> EIP712Domain {
    EIP712Domain {
        name: Some("DEX".to_string()),
        version: Some("0.1.0".to_string()),
        chain_id: Some(chain_id),
        verifying_contract: Some(verifying_contract),
        salt: None,
    }
}

mod tests {

    #[test]
    fn test_default_subaccount() -> Result<()> {
        let subaccount = default_subaccount("0x389Bd6EF1a2E399326Fa6B4AcCEB22D411b8b224")?;
        println!("subaccount: {:02x?}", subaccount);
        Ok(())
    }

    #[test]
    fn test_to_bytes_macro() -> Result<()> {
        let s = "default";
        let bytes = to_bytes!(s, 12);

        println!("bytes: {:?}", bytes);
        Ok(())
    }

    #[test]
    fn test_domain() -> Result<()> {
        let chain_id = U256::from(1);
        let verifying_contract = H160::from_str("0x46B86BA4f2d4C04975a110B85f54fe60D418388C")?;
        let domain = domain(chain_id, verifying_contract);
        Ok(())
    }

    #[test]
    fn test_get_eip712_digest() -> Result<()> {
        // 0x0d7E94332B2BCEAd8f8A761C9Ae5E5d3Cbe10bD6
        //   0x16fa1671a0a294c728e0e1e0fe329f7d976555f7bb253833bf0f18cc8065305e
        let subaccount = default_subaccount("0xEF7AdD5a2001c6f97D45141625670BB32c19425A")?;
        let order = bindings::i_endpoint::Order {
            sender: subaccount,
            price_x18: 0,
            amount: 0,
            expiration: 0,
            nonce: 0,
        };
        let hash = order.encode_eip712()?;
        println!("hash: {:?}", hash);

        Ok(())
    }
}
