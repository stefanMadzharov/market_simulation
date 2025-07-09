use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Trade<C, I: Clone> {
    pub commodity: C,
    pub volume: f32,
    pub price: Decimal,
    pub timestamp: DateTime<FixedOffset>,
    pub seller: I,
    pub buyer: I,
}
