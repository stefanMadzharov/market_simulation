use crate::{
    bot::Bot,
    bot_components::{risk_aversion::IsRiskAversion, sentiment::IsSentiment},
    strategies::decision::Decision,
};
use core::structs::market::IsCommodity;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub trait IsStrategy<C, S, RA>
where
    C: IsCommodity,
    S: IsSentiment,
    RA: IsRiskAversion,
{
    fn decide(bot: &Bot<C, S, RA>, prices: &HashMap<C, Decimal>) -> HashMap<C, Decision>;
}
