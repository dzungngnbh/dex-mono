use anyhow::{bail, Result};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use skiplist::OrderedSkipList;
use std::cell::LazyCell;

use crate::orderbook::order::Order;
use crate::orderbook::order_command::{OrderCommand, OrderType};

type OrderIndexType = DashMap<[u8; 32], Order>;
type OrderListType = OrderedSkipList<Order>;

/// list of orderbooks
pub static mut ORDERBOOKS: LazyCell<DashMap<u32, OrderBook>> = LazyCell::new(|| {
    let mut map = DashMap::new();
    map.insert(3, OrderBook::new(3));
    map
});

// each orderbook will be created for each product on chain
#[derive(Debug)]
pub struct OrderBook {
    pub product_id: u32,
    buy_orders: OrderListType,
    sell_orders: OrderListType,

    orders_index: OrderIndexType,
}

impl OrderBook {
    pub fn get(product_id: u32) -> Result<Ref<'static, u32, OrderBook>> {
        let orderbooks = unsafe { &ORDERBOOKS };
        let orderbook = orderbooks.get(&product_id);
        if orderbook.is_none() {
            return bail!("Orderbook not found");
        }

        Ok(orderbook.unwrap())
    }

    fn new(product_id: u32) -> Self {
        let sell_orders = unsafe {
            OrderedSkipList::with_comp(|a: &Order, b: &Order| {
                let res = b.price.partial_cmp(&a.price).unwrap();
                if res != std::cmp::Ordering::Equal {
                    return res;
                }
                b.timestamp.cmp(&a.timestamp)
            })
        };

        // TODO: laod the persisted orders from clickhouse

        Self {
            product_id,
            buy_orders: OrderedSkipList::new(),
            sell_orders,

            orders_index: DashMap::new(),
        }
    }

    // order_command should be valid before place_order
    // TODO: remove order_command and use Order only
    pub fn place_order(&mut self, order_command: OrderCommand) -> Result<bool> {
        // tryMatchInstantly: only for CLOBS, the inserted order will be matched using interval in case of FBA.
        if order_command.order_type == OrderType::GTC {
            self.place_order_gtc(order_command)?;
        }

        Ok(true)
    }

    fn insert_order(&mut self, order: &Order) -> Result<()> {
        let order_digest = order.order_digest;
        self.orders_index.insert(order_digest, order.clone());

        let order_list = match order.amount > 0.0 {
            true => &mut self.buy_orders,
            false => &mut self.sell_orders,
        };

        order_list.insert(order.clone());

        Ok(())
    }

    fn place_order_gtc(&mut self, order_command: OrderCommand) -> Result<()> {
        self.insert_order(&order_command.order)?;

        Ok(())
    }

    // FBA implementation
    pub fn try_match_orders(&mut self) -> Result<()> {
        // checking if prices are crossed
        if (self.buy_orders.is_empty() || self.sell_orders.is_empty())
            || (self.sell_orders.back().unwrap().price > self.buy_orders.front().unwrap().price)
        // not crossing
        {
            return Ok(());
        }

        // final is batching orders and submit them to chain

        // loop through sell_orders and buy_orders and match them if possible
        let mut best_sell_order = self.sell_orders.back().unwrap();
        let mut best_buy_order = self.buy_orders.front().unwrap();

        // stats recording for debugging, for production, we will collect ethereum logs to get the stats
        let mut matched_order = 0; // for example just increase
        let mut matched_amount = 0.0;

        let mut next_best_sell_order: Option<&Order> = None;
        let mut next_best_buy_order: Option<&Order> = None;
        loop {
            if best_sell_order.price > best_buy_order.price {
                break;
            }

            if best_sell_order.price <= best_buy_order.price {
                matched_order += 1;

                let sell_order_amount_abs = best_sell_order.amount.abs();
                let buy_order_amount_abs = best_buy_order.amount.abs();
                // compare to see what orders will be reduced and what orders will be removed
                if sell_order_amount_abs > buy_order_amount_abs {
                    // sell_order will be reduced
                    // get last order
                    // best_sell_order.chain_order.amount += buy_order_amount_abs;
                    // buy_order will be removed
                    matched_amount += buy_order_amount_abs;
                    // remove buy_order from buy_orders
                    // best_buy_order = self.buy_orders.pop_front();
                    // update best_buy_order
                    self.buy_orders.pop_front();
                    next_best_buy_order = self.buy_orders.front();
                    if next_best_buy_order.is_none() {
                        break;
                    } else {
                        best_buy_order = &next_best_buy_order.unwrap();
                    }
                } else if sell_order_amount_abs < buy_order_amount_abs {
                    // sell_order will be removed
                    // buy_order will be reduced
                    matched_amount += sell_order_amount_abs;
                    // remove sell_order from sell_orders
                    self.sell_orders.pop_back();
                    next_best_sell_order = self.sell_orders.back();
                    if next_best_sell_order.is_none() {
                        break;
                    } else {
                        best_sell_order = &next_best_sell_order.unwrap();
                    }
                    // update best_sell_order
                } else {
                    // both orders will be removed
                    matched_amount += sell_order_amount_abs;
                    // update best_sell_order
                    self.sell_orders.pop_back();
                    self.buy_orders.pop_front();
                    next_best_sell_order = self.sell_orders.back();
                    next_best_buy_order = self.buy_orders.front();
                    if next_best_sell_order.is_none() || next_best_buy_order.is_none() {
                        break;
                    } else {
                        best_sell_order = &next_best_sell_order.unwrap();
                        best_buy_order = &next_best_buy_order.unwrap();
                    }
                }

                if next_best_sell_order.is_none() || next_best_buy_order.is_none() {
                    break;
                }
            }
        }

        println!("matched_order: {}", matched_order);
        println!("matched_amount: {}", matched_amount);

        Ok(())
    }

    // utils
    pub fn print_debug(&self) {
        println!("sell_orders:");
        self.sell_orders
            .iter()
            .for_each(|order| order.print_debug());

        println!("buy_orders:");
        self.buy_orders.iter().for_each(|order| order.print_debug());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dashmap::DashMap;
    use dex_client::dex_utils::bindings;

    type bytes32 = [u8; 32];

    fn mock_order() -> Order {
        Order {
            amount: 1.0,
            price: 1.0,
            order_type: OrderType::GTC,
            product_id: 1,
            order_digest: [0u8; 32],
            timestamp: 0,
        }
    }

    fn load_orders(order_book: &mut OrderBook) -> Result<()> {
        let mut sell_order1 = mock_order();
        sell_order1.amount = -1.0;
        sell_order1.price = 500.0;
        order_book.insert_order(&sell_order1)?;
        let mut sell_order2 = mock_order();
        sell_order2.amount = -2.0;
        sell_order2.price = 1000.0;
        sell_order2.timestamp = 1;
        order_book.insert_order(&sell_order2)?;
        let mut sell_order3 = mock_order();
        sell_order3.amount = -2.0;
        sell_order3.price = 1000.0;
        sell_order3.timestamp = 2; // later should be behind
        order_book.insert_order(&sell_order3)?;

        let mut order1 = mock_order();
        order1.amount = 1.0;
        order1.price = 500.0;
        order_book.insert_order(&order1)?;
        let mut order2 = mock_order();
        order2.amount = 2.0;
        order2.price = 300.0;
        order2.timestamp = 1;
        order_book.insert_order(&order2)?;
        let mut order3 = mock_order();
        order3.amount = 2.0;
        order3.price = 300.0;
        order3.timestamp = 2; // later should be behind
        order_book.insert_order(&order3)?;

        Ok(())
    }

    #[test]
    fn test_dashmap() -> Result<()> {
        let reviews = DashMap::<bytes32, Order>::new();
        let k = [0u8; 32];
        let mut order = mock_order();
        order.amount = 1.0;

        reviews.insert(k, order);

        let v = reviews.get(&k).unwrap();

        Ok(())
    }

    #[test]
    fn test_place_order() -> Result<()> {
        let mut order_book = OrderBook::new(3);
        let mut order = mock_order();
        order.amount = -1.0;
        order_book.place_order(OrderCommand::new(OrderType::GTC, order)?)?;

        let mut market_order = mock_order();
        market_order.amount = -1.0;
        order_book.place_order(OrderCommand::new(OrderType::Market, market_order)?)?;

        Ok(())
    }

    #[test]
    fn test_insert_order() -> Result<()> {
        let mut order_book = OrderBook::new(3);
        let mut order = mock_order();
        order.amount = -1.0;
        order_book.insert_order(&order)?;

        Ok(())
    }

    #[test]
    fn test_buy_sell_orders_list() -> Result<()> {
        let mut order_book = OrderBook::new(3);
        load_orders(&mut order_book)?;
        order_book.print_debug();

        Ok(())
    }

    #[test]
    fn test_try_match_orders() -> Result<()> {
        let mut order_book = OrderBook::new(3);
        load_orders(&mut order_book)?;
        println!("before try_match_orders");
        order_book.print_debug();
        order_book.try_match_orders()?;
        println!("after try_match_orders");
        order_book.print_debug();

        Ok(())
    }

    #[test]
    fn test_get_orderbook() -> Result<()> {
        let orderbook = OrderBook::get(3)?;
        dbg!(&orderbook.product_id);
        Ok(())
    }
}
