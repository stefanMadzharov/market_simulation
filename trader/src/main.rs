use core::{matching_logic::fifo::FIFO, structs::market::Market};
use rust_decimal::dec;

fn main() {
    let mut market = Market::new(FIFO {});
    market.place_buy_order("Gold", 1.0, dec!(1.0), &"Buyer1".to_string());
    market.place_buy_order("Wood", 1.0, dec!(1.0), &"Buyer1".to_string());
    let trades = market.match_trades();
    dbg!(&trades);
}
