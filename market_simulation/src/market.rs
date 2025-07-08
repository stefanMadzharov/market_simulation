use core::{
    matching_logic::{MatchingAlgorithm, Trade},
    order_book::OrderBook,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Market<C, I, MA>
where
    C: Eq + PartialEq + Hash + Clone,
    I: Clone,
    MA: MatchingAlgorithm<C, I>,
{
    order_books: HashMap<C, OrderBook<C, I>>,
    matching_algorithm: MA,
}

impl<C, I, MA> Market<C, I, MA>
where
    C: Eq + PartialEq + Hash + Clone,
    I: Clone,
    MA: MatchingAlgorithm<C, I>,
{
    pub fn new(matching_algorithm: MA) -> Self {
        Self {
            order_books: HashMap::new(),
            matching_algorithm,
        }
    }

    pub fn place_buy_order(&mut self, commodity: C, volume: f32, price: Decimal, initiator: &I) {
        self.order_books
            .entry(commodity.clone())
            .and_modify(|order_book| order_book.add_buy(volume, price, initiator))
            .or_insert_with(|| {
                let mut order_book = OrderBook::<C, I>::with_commodity(commodity);
                order_book.add_buy(volume, price, initiator);
                order_book
            });
    }

    pub fn place_sell_order(&mut self, commodity: C, volume: f32, price: Decimal, initiator: &I) {
        self.order_books
            .entry(commodity.clone())
            .and_modify(|order_book| order_book.add_sell(volume, price, initiator))
            .or_insert_with(|| {
                let mut order_book = OrderBook::<C, I>::with_commodity(commodity);
                order_book.add_sell(volume, price, initiator);
                order_book
            });
    }

    pub fn match_trades(&mut self) -> HashMap<C, Vec<Trade<C, I>>> {
        let mut all_trades = HashMap::new();
        for (commodity, order_book) in &mut self.order_books {
            all_trades.insert(
                commodity.clone(),
                order_book.match_trades(&self.matching_algorithm),
            );
        }
        all_trades
    }
}
