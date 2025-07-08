use core::matching_logic::FIFO;
use market_simulation::market::Market;
use rust_decimal::dec;

fn main() {
    let mut market = Market::new(FIFO {});

    market.place_buy_order("Gold", 1.0, dec!(1.0), &"A".to_owned());
    market.place_sell_order("Gold", 1.0, dec!(1.0), &"B".to_owned());
    market.place_buy_order("Gold", 2.0, dec!(1.5), &"C".to_owned());
    market.place_sell_order("Gold", 2.0, dec!(1.5), &"D".to_owned());

    market.place_buy_order("Wood", 1.0, dec!(1.0), &"1".to_owned());
    market.place_sell_order("Wood", 1.0, dec!(1.0), &"2".to_owned());
    market.place_buy_order("Wood", 2.0, dec!(1.5), &"3".to_owned());
    market.place_sell_order("Wood", 2.0, dec!(1.5), &"4".to_owned());

    market.place_buy_order("Gold", 5.0, dec!(1.0), &"E".to_owned());
    market.place_sell_order("Gold", 5.0, dec!(1.0), &"F".to_owned());
    market.place_buy_order("Gold", 5.0, dec!(1.5), &"G".to_owned());
    market.place_sell_order("Gold", 5.0, dec!(1.5), &"H".to_owned());

    dbg!(&market);

    let trades = market.match_trades();

    dbg!(trades);
}
