use anyhow::Result;
use dashmap::DashMap;
use std::sync::LazyLock;

static TRADE_SYMBOLS: LazyLock<DashMap<&str, u32>> = LazyLock::new(|| {
    let mut map = DashMap::new();
    map.insert("BTC_DAI", 3);
    map
});

pub fn get_product_id(symbol: &str) -> Result<u32> {
    let map = TRADE_SYMBOLS.get(symbol).unwrap();
    Ok(*map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_id() {
        let product_id = get_product_id("BTC_DAI").unwrap();
        assert_eq!(product_id, 3);
    }
}
