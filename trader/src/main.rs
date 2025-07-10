use rust_decimal::{dec, Decimal};
use std::collections::HashMap;
use trader::{
    bot::Bot,
    bot_components::{risk_aversion::RiskAversion, sentiment::Sentiment},
    strategies::{is_strategy::IsStrategy, random_strategy::RandomStrategy},
    trader::Trader,
};

fn main() {
    let trader = Trader {
        balance: dec!(1000),
        commodities_volume: [("Gold", dec!(5)), ("Wood", dec!(5))]
            .into_iter()
            .collect::<HashMap<&'static str, Decimal>>(),
    };
    let random_bot = Bot {
        sentiment: Sentiment::Optimistic,
        risk_aversion: RiskAversion::Risky,
        trader,
    };
    let prices = [("Gold", dec!(20)), ("Wood", dec!(10))]
        .into_iter()
        .collect();
    let positions = RandomStrategy::decide(&random_bot, &prices);
    dbg!(positions);
}
