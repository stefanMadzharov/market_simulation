use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct Trader<C> {
    pub balance: Decimal,
    pub commodities: HashMap<C, f32>,
}
