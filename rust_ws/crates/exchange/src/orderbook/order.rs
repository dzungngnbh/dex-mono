use crate::orderbook::order_command::OrderType;
use dex_client::dex_utils::bindings;
use std::cmp::Ordering;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Invalid order")]
    InvalidOrder,
}

// the only reason we have this is to not messing with IEndpoint order
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub amount: f64,
    pub order_digest: [u8; 32],
    pub order_type: OrderType,
    pub price: f64,
    pub product_id: u32,
    pub timestamp: u64,
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = other.price.partial_cmp(&self.price)?;
        if res != Ordering::Equal {
            return Some(res);
        }
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl Order {
    pub fn is_valid(&self) -> bool {
        return self.amount != 0.0 && self.product_id != 0; // not the quote
                                                           // && self.order_digest != [0; 32]; // we take the signing part properly later
    }

    pub fn to_chain(&self) -> bindings::i_offchain_orderbook::Order {
        bindings::i_offchain_orderbook::Order {
            sender: [0u8; 32],
            // TODO: Fix this
            price_x18: (self.price * 1e18) as i128,
            amount: (self.amount * 1e18) as i128,
            expiration: 0,
            nonce: 0,
        }
    }

    // only prints amount, price, submit_timestamp
    pub fn print_debug(&self) {
        println!("{} | {} | {}", self.amount, self.price, self.timestamp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        let order1 = Order {
            amount: 1.0,
            order_digest: [0; 32],
            order_type: OrderType::GTC,
            price: 1.0,
            product_id: 1,
            timestamp: 0,
        };
        let order2 = Order {
            amount: 2.0,
            order_digest: [0; 32],
            order_type: OrderType::GTC,
            price: 2.0,
            product_id: 1,
            timestamp: 0,
        };

        let res = order2 > order1;
        println!("res: {:?}", res);
    }
}
