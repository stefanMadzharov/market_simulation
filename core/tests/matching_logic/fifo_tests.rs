use core::{matching_logic::fifo::FIFO, structs::market::Market};
use rust_decimal::dec;

#[test]
fn test_simple_match() {
    let mut market = Market::new(FIFO {});
    market.place_buy_order("Gold", 1.0, dec!(10.0), &"Buyer1".to_string());
    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller1".to_string());

    let trades = market.match_trades();
    assert_eq!(trades["Gold"].len(), 1);
    let trade = &trades["Gold"][0];
    assert_eq!(trade.buyer, "Buyer1");
    assert_eq!(trade.seller, "Seller1");
    assert_eq!(trade.volume, 1.0);
    assert_eq!(trade.price, dec!(10.0));
}

#[test]
fn test_no_match_due_to_price() {
    let mut market = Market::new(FIFO {});
    market.place_buy_order("Gold", 1.0, dec!(9.0), &"Buyer1".to_string());
    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller1".to_string());

    let trades = market.match_trades();
    assert!(trades["Gold"].is_empty());
}

#[test]
fn test_partial_match() {
    let mut market = Market::new(FIFO {});
    market.place_buy_order("Gold", 2.0, dec!(10.0), &"Buyer1".to_string());
    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller1".to_string());

    let trades = market.match_trades();
    assert_eq!(trades["Gold"].len(), 1);
    assert_eq!(trades["Gold"][0].volume, 1.0);
}

#[test]
fn test_multiple_orders_fifo_priority() {
    let mut market = Market::new(FIFO {});

    // Two sell orders
    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller1".to_string());
    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller2".to_string());

    // One buy order that can match both
    market.place_buy_order("Gold", 2.0, dec!(10.0), &"Buyer".to_string());

    let trades = market.match_trades();
    assert_eq!(trades["Gold"].len(), 2);

    assert_eq!(trades["Gold"][0].seller, "Seller1");
    assert_eq!(trades["Gold"][1].seller, "Seller2");
}

#[test]
fn test_crossed_orders_all_match() {
    let mut market = Market::new(FIFO {});
    market.place_sell_order("Gold", 1.0, dec!(5.0), &"Seller".to_string());
    market.place_buy_order("Gold", 1.0, dec!(10.0), &"Buyer".to_string());

    let trades = market.match_trades();
    assert_eq!(trades["Gold"].len(), 1);
    assert_eq!(trades["Gold"][0].price, dec!(7.5));
}

#[test]
fn test_same_price_different_times_fifo() {
    let mut market = Market::new(FIFO {});

    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller1".to_string());
    market.place_sell_order("Gold", 1.0, dec!(10.0), &"Seller2".to_string());
    market.place_buy_order("Gold", 1.0, dec!(10.0), &"Buyer1".to_string());
    market.place_buy_order("Gold", 1.0, dec!(10.0), &"Buyer2".to_string());

    let trades = market.match_trades();
    assert_eq!(trades["Gold"].len(), 2);
    assert_eq!(trades["Gold"][0].buyer, "Buyer1");
    assert_eq!(trades["Gold"][0].seller, "Seller1");
    assert_eq!(trades["Gold"][1].buyer, "Buyer2");
    assert_eq!(trades["Gold"][1].seller, "Seller2");
}

#[test]
fn test_multiple_commodities() {
    let mut market = Market::new(FIFO {});
    market.place_buy_order("Gold", 1.0, dec!(10.0), &"BuyerGold".to_string());
    market.place_sell_order("Wood", 1.0, dec!(5.0), &"SellerWood".to_string());

    let trades = market.match_trades();
    assert!(trades["Gold"].is_empty());
    assert!(trades["Wood"].is_empty());
}

#[test]
fn test_big_mumber_of_matches() {
    let mut market = Market::new(FIFO {});

    for _ in 0..900 {
        market.place_buy_order("Gold", 1.0, dec!(1.0), &"Buyer1".to_string());
        market.place_sell_order("Gold", 1.0, dec!(1.0), &"Seller1".to_string());
        market.place_buy_order("Gold", 2.0, dec!(1.5), &"Buyer2".to_string());
        market.place_sell_order("Gold", 2.0, dec!(1.5), &"Seller2".to_string());

        market.place_buy_order("Wood", 1.0, dec!(1.0), &"Buyer1".to_string());
        market.place_sell_order("Wood", 1.0, dec!(1.0), &"Seller1".to_string());
        market.place_buy_order("Wood", 2.0, dec!(1.5), &"Buyer1".to_string());
        market.place_sell_order("Wood", 2.0, dec!(1.5), &"Seller1".to_string());

        market.place_sell_order("Something Common", 2.0, dec!(1.5), &"Seller1".to_string());
        market.place_buy_order("Something Rare", 2.0, dec!(1.5), &"Buyer1".to_string());

        market.place_buy_order("Gold", 5.0, dec!(1.0), &"Buyer1".to_string());
        market.place_sell_order("Gold", 5.0, dec!(1.0), &"Seller1".to_string());
        market.place_buy_order("Gold", 5.0, dec!(1.5), &"Buyer1".to_string());
        market.place_sell_order("Gold", 5.0, dec!(1.5), &"Seller1".to_string());
    }

    market.match_trades();
}
