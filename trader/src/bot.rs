use core::structs::market::IsCommodity;

use crate::{
    bot_components::{risk_aversion::IsRiskAversion, sentiment::IsSentiment},
    trader::Trader,
};

pub struct Bot<C, S, RA>
where
    C: IsCommodity,
    S: IsSentiment,
    RA: IsRiskAversion,
{
    pub sentiment: S,
    pub risk_aversion: RA,
    pub trader: Trader<C>,
}
