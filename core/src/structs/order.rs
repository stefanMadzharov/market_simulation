use chrono::{DateTime, FixedOffset, Local};
use rust_decimal::Decimal;
use std::{fmt, marker::PhantomData};

#[derive(Clone)]
pub struct Order<OT: IsOrderType + Clone, I: Clone> {
    pub volume: Decimal,
    pub price: Decimal,
    pub timestamp: DateTime<FixedOffset>,
    pub initiator: I,
    order_type: PhantomData<fn() -> OT>,
}

impl<OT: IsOrderType + Clone, I: Clone + fmt::Debug> fmt::Debug for Order<OT, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Order")
            .field("volume", &self.volume)
            .field("price", &self.price)
            .field("timestamp", &self.timestamp)
            .field("initiator", &self.initiator)
            .finish()
    }
}

impl<OT: IsOrderType + Clone, I: Clone> Order<OT, I> {
    pub fn new(volume: Decimal, price: Decimal, initiator: &I) -> Self {
        Self {
            volume,
            price,
            initiator: initiator.clone(),
            order_type: PhantomData::<fn() -> OT>,
            timestamp: Local::now().into(),
        }
    }
}

pub trait IsOrderType: fmt::Debug {}

#[derive(Debug, Clone, Copy)]
pub struct Buy;
impl IsOrderType for Buy {}

#[derive(Debug, Clone, Copy)]
pub struct Sell;
impl IsOrderType for Sell {}
