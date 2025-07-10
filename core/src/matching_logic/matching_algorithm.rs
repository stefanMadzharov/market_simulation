use crate::structs::{market::IsCommodity, order_book::OrderBook, trade::Trade};

pub trait MatchingAlgorithm<C: IsCommodity, I: Clone> {
    fn execute_trades(order_book: &mut OrderBook<C, I>) -> Vec<Trade<C, I>>;
}
