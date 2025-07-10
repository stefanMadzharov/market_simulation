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
use rust_decimal::{
    dec,
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use std::collections::HashMap;

struct RandomStrategy {}

impl<C, S, RA> IsStrategy<C, S, RA> for RandomStrategy
where
    C: IsCommodity,
    S: IsSentiment,
    RA: IsRiskAversion,
{
    fn decide(bot: &Bot<C, S, RA>) -> HashMap<C, Decision> {
        let mut rng = rng();
        let mut decisions = HashMap::new();

        let sentiment_score = bot.sentiment.calculate();
        let risk_score = bot.risk_aversion.calculate();

        for (commodity, &volume) in bot.trader.commodities.iter() {
            if volume == 0.0 {
                decisions.insert(commodity.clone(), Decision::Holding);
                continue;
            }

            let sentiment_f64 = sentiment_score.to_f64().unwrap_or(0.5);
            let risk_f64 = risk_score.to_f64().unwrap_or(0.5);
            let dice: f64 = rng.random();

            let decision = if (sentiment_f64 - 0.5).abs() < f64::EPSILON {
                Decision::Holding
            } else if sentiment_f64 > 0.5 && dice < (risk_f64 - 0.5) {
                // Buy
                let amount = rng.random_range(
                    0.0..=(bot.trader.balance / Decimal::from(bot.trader.commodities.len() as u32))
                        .to_f64()
                        .unwrap_or(0.0),
                );
                Decision::Buying {
                    position: Position {
                        price: todo!(),  //Decimal::from_f64(rng.random_range(0.8..1.2)).unwrap(),
                        volume: todo!(), //amount as f32,
                    },
                }
            } else if sentiment_f64 < 0.5 && dice < (risk_f64 - 0.5) {
                let amount = rng.random_range(0.0..=volume.min(1.0));
                Decision::Selling {
                    position: Position {
                        price: todo!(), // or: Decimal::from_f64(rng.gen_range(0.8..1.2)).unwrap()
                        volume: todo!(),
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
