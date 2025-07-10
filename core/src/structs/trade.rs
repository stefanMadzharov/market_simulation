use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

use crate::structs::market::IsCommodity;

#[derive(Debug)]
pub struct Trade<C: IsCommodity, I: Clone> {
    pub commodity: C,
    pub volume: Decimal,
    pub price: Decimal,
    pub timestamp: DateTime<FixedOffset>,
    pub seller: I,
    pub buyer: I,
}
