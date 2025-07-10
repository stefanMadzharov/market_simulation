use core::structs::market::IsCommodity;

use crate::{
    bot_components::{risk_aversion::IsRiskAversion, sentiment::IsSentiment},
    trader::Trader,
};

struct Bot<C, S, RA>
where
    C: IsCommodity,
    S: IsSentiment,
    RA: IsRiskAversion,
{
    sentiment: S,
    risk_aversion: RA,
    trader: Trader<C>,
}
