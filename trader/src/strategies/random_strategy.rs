use crate::{
    bot::Bot,
    bot_components::{risk_aversion::IsRiskAversion, sentiment::IsSentiment},
    strategies::{
        decision::{Decision, Position},
        is_strategy::IsStrategy,
    },
};
use core::structs::market::IsCommodity;
use rand::{rng, Rng};
use rust_decimal::{dec, prelude::FromPrimitive, Decimal};
use std::collections::HashMap;

pub struct RandomStrategy {}

impl<C, S, RA> IsStrategy<C, S, RA> for RandomStrategy
where
    C: IsCommodity,
    S: IsSentiment,
    RA: IsRiskAversion,
{
    fn decide(bot: &Bot<C, S, RA>, prices: &HashMap<C, Decimal>) -> HashMap<C, Decision> {
        let mut rng = rng();
        let mut decisions = HashMap::new();

        let sentiment_score = bot.sentiment.calculate();
        let risk_score = bot.risk_aversion.calculate();

        for (commodity, orig_price) in prices.iter() {
            let dice = Decimal::from_f64(rng.random::<f64>()).unwrap();

            let average_balance_per_commodity_left =
                bot.trader.balance / Decimal::from(prices.len());
            let position_price =
                Decimal::from_f64(rng.random_range(0.8..1.2)).unwrap_or_default() * orig_price;
            let risk_adjusted_position_price = position_price * risk_score.abs() * dec!(2);
            let volume = average_balance_per_commodity_left / risk_adjusted_position_price;

            let decision = if sentiment_score <= dice && sentiment_score.is_sign_positive() {
                Decision::Buying {
                    position: Position {
                        price: risk_adjusted_position_price,
                        volume,
                    },
                }
            } else if sentiment_score.abs() <= dice && sentiment_score.is_sign_negative() {
                Decision::Selling {
                    position: Position {
                        price: risk_adjusted_position_price.abs(),
                        volume: volume.abs(),
                    },
                }
            } else {
                Decision::Holding
            };

            decisions.insert(commodity.clone(), decision);
        }

        decisions
    }
}
