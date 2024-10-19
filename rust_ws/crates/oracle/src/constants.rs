pub const DAI_QUOTE_PRODUCT_ID: u32 = 0;

pub const BTCUSD_INTERNAL_ID: u32 = 3;
pub const BTC_USD_ORACLE_ID: &str =
    "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43";

pub fn is_quote_product(product_id: u32) -> bool {
    product_id == DAI_QUOTE_PRODUCT_ID
}
