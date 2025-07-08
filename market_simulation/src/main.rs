use market_simulation::market::Market;
use rust_decimal::dec;

fn main() {
    let mut market = Market::new();

    market.place_buy_order("Gold", 1.0, dec!(1.0));
    market.place_sell_order("Gold", 1.0, dec!(1.0));
    market.place_buy_order("Gold", 2.0, dec!(1.5));
    market.place_sell_order("Gold", 2.0, dec!(1.5));

    market.place_buy_order("Wood", 1.0, dec!(1.0));
    market.place_sell_order("Wood", 1.0, dec!(1.0));
    market.place_buy_order("Wood", 2.0, dec!(1.5));
    market.place_sell_order("Wood", 2.0, dec!(1.5));

    market.place_sell_order("Something Common", 2.0, dec!(1.5));
    market.place_buy_order("Something Rare", 2.0, dec!(1.5));

    market.place_buy_order("Gold", 5.0, dec!(1.0));
    market.place_sell_order("Gold", 5.0, dec!(1.0));
    market.place_buy_order("Gold", 5.0, dec!(1.5));
    market.place_sell_order("Gold", 5.0, dec!(1.5));

    dbg!(market);
}
