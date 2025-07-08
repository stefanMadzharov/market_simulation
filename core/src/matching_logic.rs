// This file is for different matching logics TODO
// 1. Price-Time Priority/FIFO
// 2. Pro-Rata
// 3. Randomized
// 4. Frequent Batch Auctions
// 5. Dutch Auction Matching
use chrono::{DateTime, FixedOffset, Local};
use rust_decimal::{dec, Decimal};

use crate::order_book::OrderBook;

#[derive(Debug)]
pub struct Trade<C, I: Clone> {
    pub commodity: C,
    pub volume: f32,
    pub price: Decimal,
    pub timestamp: DateTime<FixedOffset>,
    pub seller: I,
    pub buyer: I,
    //TODO maybe add From/To or something like that
}

pub trait MatchingAlgorithm<C, I: Clone> {
    fn execute_trades(order_book: &mut OrderBook<C, I>) -> Vec<Trade<C, I>>;
}

#[derive(Debug)]
pub struct FIFO {}
impl<C: Clone, I: Clone> MatchingAlgorithm<C, I> for FIFO {
    fn execute_trades(order_book: &mut OrderBook<C, I>) -> Vec<Trade<C, I>> {
        let mut trades = Vec::new();
        let sell_prices: Vec<_> = order_book.sell_orders.keys().cloned().collect();
        let buy_prices: Vec<_> = order_book.buy_orders.keys().rev().cloned().collect();

        'sell_loop: for sell_price in sell_prices {
            let sell_queue = order_book.sell_orders.get_mut(&sell_price).unwrap();

            let mut i = 0;
            while i < sell_queue.len() {
                let sell_order = &mut sell_queue[i];

                let mut matched = false;

                for buy_price in &buy_prices {
                    if *buy_price < sell_order.price {
                        break 'sell_loop;
                    }

                    let buy_queue = order_book.buy_orders.get_mut(buy_price).unwrap();
                    let mut j = 0;

                    while j < buy_queue.len() {
                        let buy_order = &mut buy_queue[j];

                        let trade_volume = sell_order.volume.min(buy_order.volume);
                        if trade_volume <= 0.0 {
                            j += 1;
                            continue;
                        }

                        trades.push(Trade {
                            commodity: order_book.commodity.clone(),
                            price: (*buy_price + sell_price) / dec!(2), //TODO update with price_determination algorithm
                            volume: trade_volume,
                            seller: sell_order.initiator.clone(),
                            buyer: buy_order.initiator.clone(),
                            timestamp: Local::now().into(),
                        });

                        sell_order.volume -= trade_volume;
                        buy_order.volume -= trade_volume;

                        if buy_order.volume <= 0.0 {
                            buy_queue.remove(j);
                        } else {
                            j += 1;
                        }

                        matched = true;
                    }

                    if buy_queue.is_empty() {
                        order_book.buy_orders.remove(buy_price);
                    }

                    if sell_order.volume == 0.0 {
                        break;
                    }
                }

                if sell_order.volume == 0.0 {
                    sell_queue.remove(i);
                } else {
                    if !matched {
                        i += 1;
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
