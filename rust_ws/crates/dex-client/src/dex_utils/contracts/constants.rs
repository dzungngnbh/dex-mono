use dashmap::DashMap;
use std::cell::LazyCell;

pub const BTC_DAI_PRODUCT_ID: u32 = 3;

// decimals
pub const PRODUCT_DECIMALS: LazyCell<DashMap<u32, u32>> = LazyCell::new(|| {
    let map = DashMap::new();
    map.insert(0, 18); // quote product which is DAI
    map.insert(BTC_DAI_PRODUCT_ID, 18);
    map
});
