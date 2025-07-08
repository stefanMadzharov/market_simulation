use core::order_book::OrderBook;
use rust_decimal::Decimal;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Market<C: Eq + PartialEq + Hash + Clone> {
    order_books: HashMap<C, OrderBook<C>>,
}

impl<C: Eq + PartialEq + Hash + Clone> Market<C> {
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
        }
    }

    pub fn place_buy_order(&mut self, commodity: C, volume: f32, price: Decimal) {
        self.order_books
            .entry(commodity.clone())
            .and_modify(|order_book| order_book.add_buy(volume, price))
            .or_insert_with(|| {
                let mut order_book = OrderBook::<C>::with_commodity(commodity);
                order_book.add_buy(volume, price);
                order_book
            });
    }

    pub fn place_sell_order(&mut self, commodity: C, volume: f32, price: Decimal) {
        self.order_books
            .entry(commodity.clone())
            .and_modify(|order_book| order_book.add_sell(volume, price))
            .or_insert_with(|| {
                let mut order_book = OrderBook::<C>::with_commodity(commodity);
                order_book.add_sell(volume, price);
                order_book
            });
    }
}
