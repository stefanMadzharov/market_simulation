// This file is for different matching logics TODO
// 1. Price-Time Priority/FIFO
// 2. Pro-Rata
// 3. Randomized
// 4. Frequent Batch Auctions
// 5. Dutch Auction Matching
use chrono::Local;
use rust_decimal::dec;

use crate::{
    matching_logic::matching_algorithm::MatchingAlgorithm,
    structs::{order_book::OrderBook, trade::Trade},
};

#[derive(Debug)]
pub struct FIFO {}
impl<C: Clone + std::fmt::Debug, I: Clone + std::fmt::Debug> MatchingAlgorithm<C, I> for FIFO {
    fn execute_trades(order_book: &mut OrderBook<C, I>) -> Vec<Trade<C, I>> {
        let mut trades = Vec::new();
        let sell_prices: Vec<_> = order_book.sell_orders.keys().cloned().collect();
        let buy_prices: Vec<_> = order_book.buy_orders.keys().rev().cloned().collect();
        let mut buy_prices_removed = 0;
        if sell_prices.is_empty() || buy_prices.is_empty() {
            return trades;
        }

        'sell_price_loop: for sell_price in sell_prices {
            let sell_queue = order_book.sell_orders.get_mut(&sell_price).unwrap();

            'sell_queue_loop: while let Some(sell_order) = sell_queue.get_mut(0) {
                dbg!(&buy_prices);
                for buy_price in buy_prices.iter().cloned().skip(buy_prices_removed) {
                    if buy_price < sell_order.price {
                        break 'sell_price_loop;
                    }

                    let buy_queue = order_book.buy_orders.get_mut(&buy_price).unwrap();

                    while let Some(buy_order) = buy_queue.get_mut(0) {
                        let trade_volume = sell_order.volume.min(buy_order.volume);
                        assert!(trade_volume > 0.0);

                        trades.push(Trade {
                            commodity: order_book.commodity.clone(),
                            price: (buy_price + sell_price) / dec!(2), //TODO update with price_determination algorithm
                            volume: trade_volume,
                            seller: sell_order.initiator.clone(),
                            buyer: buy_order.initiator.clone(),
                            timestamp: Local::now().into(),
                        });

                        sell_order.volume -= trade_volume;
                        buy_order.volume -= trade_volume;

                        if buy_order.volume == 0.0 {
                            buy_queue.remove(0);
                            if buy_queue.is_empty() {
                                order_book.buy_orders.remove(&buy_price);
                                buy_prices_removed += 1;
                                if sell_order.volume == 0.0 {
                                    sell_queue.remove(0);
                                    continue 'sell_queue_loop;
                                }
                                break;
                            }
                        }

                        if sell_order.volume == 0.0 {
                            sell_queue.remove(0);
                            continue 'sell_queue_loop;
                        }
                    }
                }
            }

            if sell_queue.is_empty() {
                order_book.sell_orders.remove(&sell_price);
            }
        }

        trades
    }
}
