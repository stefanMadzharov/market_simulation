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
    structs::{market::IsCommodity, order_book::OrderBook, trade::Trade},
};

#[derive(Debug)]
pub struct FIFO {}
impl<C: IsCommodity, I: Clone> MatchingAlgorithm<C, I> for FIFO {
    fn execute_trades(order_book: &mut OrderBook<C, I>) -> Vec<Trade<C, I>> {
        let mut trades = Vec::new();

        if order_book.sell_orders.is_empty() || order_book.buy_orders.is_empty() {
            return trades;
        }

        let sell_prices: Vec<_> = order_book.sell_orders.keys().cloned().collect();
        let buy_prices: Vec<_> = order_book.buy_orders.keys().rev().cloned().collect();

        'sell_price_loop: for sell_price in sell_prices {
            let sell_queue = match order_book.sell_orders.get_mut(&sell_price) {
                Some(queue) => queue,
                None => break,
            };

            'sell_queue_loop: while !sell_queue.is_empty() {
                let sell_order_filled = {
                    let sell_order = sell_queue.get_mut(0).unwrap();

                    for buy_price in buy_prices.iter().cloned() {
                        if buy_price < sell_order.price {
                            break 'sell_price_loop;
                        }

                        let mut buy_queue = match order_book.buy_orders.remove(&buy_price) {
                            Some(queue) => queue,
                            None => break,
                        };

                        loop {
                            let Some(buy_order) = buy_queue.get_mut(0) else {
                                break;
                            };

                            let trade_volume = sell_order.volume.min(buy_order.volume);
                            assert!(trade_volume > 0.0);

                            trades.push(Trade {
                                commodity: order_book.commodity.clone(),
                                price: (buy_price + sell_price) / dec!(2),
                                volume: trade_volume,
                                seller: sell_order.initiator.clone(),
                                buyer: buy_order.initiator.clone(),
                                timestamp: Local::now().into(),
                            });

                            sell_order.volume -= trade_volume;
                            buy_order.volume -= trade_volume;

                            if buy_order.volume == 0.0 {
                                buy_queue.remove(0);
                            }

                            if sell_order.volume == 0.0 {
                                break;
                            }

                            if buy_queue.is_empty() {
                                break;
                            }
                        }

                        if !buy_queue.is_empty() {
                            order_book.buy_orders.insert(buy_price, buy_queue);
                        }

                        if sell_order.volume == 0.0 {
                            break;
                        }
                    }

                    sell_order.volume == 0.0
                };

                if sell_order_filled {
                    sell_queue.remove(0);
                    continue 'sell_queue_loop;
                } else {
                    break;
                }
            }

            if sell_queue.is_empty() {
                order_book.sell_orders.remove(&sell_price);
            }
        }

        trades
    }
}
