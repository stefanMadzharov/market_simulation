use crate::structs::{order_book::OrderBook, trade::Trade};

pub trait MatchingAlgorithm<C, I: Clone> {
    fn execute_trades(order_book: &mut OrderBook<C, I>) -> Vec<Trade<C, I>>;
}
