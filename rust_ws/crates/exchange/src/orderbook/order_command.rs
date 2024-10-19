use crate::orderbook::order::{Order, OrderError};
use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    GTC = 0,
    Market = 1,
    Cancel = 2,
}

#[derive(Debug)]
pub struct OrderCommand {
    pub order_type: OrderType,
    pub order: Order,
}

impl OrderCommand {
    // order should be checked if it's valid before creating order_command
    pub fn new(order_type: OrderType, order: Order) -> Result<Self> {
        if !order.is_valid() {
            bail!(OrderError::InvalidOrder);
        }

        let mut _order = order.clone();
        _order.order_type = order_type.clone();
        Ok(Self {
            order_type,
            order: _order,
        })
    }
}
