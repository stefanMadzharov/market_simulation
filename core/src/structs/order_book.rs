use crate::{
    matching_logic::matching_algorithm::MatchingAlgorithm,
    structs::{
        order::{Buy, Order, Sell},
        trade::Trade,
    },
};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, VecDeque};

#[derive(Default, Debug, Clone)]
pub struct OrderBook<C, I: Clone> {
    pub commodity: C,
    pub buy_orders: BTreeMap<Decimal, VecDeque<Order<Buy, I>>>,
    pub sell_orders: BTreeMap<Decimal, VecDeque<Order<Sell, I>>>,
    /*TODO
    // Add special types of orders:
    // Fill-or-Kill(FOK) - should be done instantly(single tick) and fully in a single execution or it gets deleted
    // All-or-None(AON) - should be done in a single execution
    // Immediate-or-cancel(IOC) - should be done instantly(single tick) or it gets deleted
     */
}

impl<C: Clone, I: Clone> OrderBook<C, I> {
    pub fn with_commodity(commodity: C) -> Self {
        Self {
            commodity,
            sell_orders: BTreeMap::new(),
            buy_orders: BTreeMap::new(),
        }
    }

    pub fn add_buy(&mut self, volume: f32, price: Decimal, initiator: &I) {
        self.buy_orders
            .entry(price)
            .and_modify(|queue| queue.push_back(Order::<Buy, I>::new(volume, price, initiator)))
            .or_insert_with(|| {
                let mut queue = VecDeque::new();
                queue.push_back(Order::<Buy, I>::new(volume, price, initiator));
                queue
            });
    }

    pub fn add_sell(&mut self, volume: f32, price: Decimal, initiator: &I) {
        self.sell_orders
            .entry(price)
            .and_modify(|queue| queue.push_back(Order::<Sell, I>::new(volume, price, initiator)))
            .or_insert_with(|| {
                let mut queue = VecDeque::new();
                queue.push_back(Order::<Sell, I>::new(volume, price, initiator));
                queue
            });
    }

    pub fn match_trades<MA>(&mut self, _matching_algorithm: &MA) -> Vec<Trade<C, I>>
    where
        MA: MatchingAlgorithm<C, I>,
    {
        <MA as MatchingAlgorithm<C, I>>::execute_trades(self)
    }
}
