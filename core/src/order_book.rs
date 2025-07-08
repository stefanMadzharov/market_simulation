use chrono::{DateTime, FixedOffset, Local};
use std::{fmt, marker::PhantomData};

#[derive(Default, Debug)]
pub struct OrderBook<C> {
    pub commodity: C,
    pub buy_orders: Vec<Order<Buy>>,
    pub sell_orders: Vec<Order<Sell>>,
    /*TODO
    // Add special types of orders:
    // Fill-or-Kill(FOK) - should be done instantly(single tick) and fully in a single execution or it gets deleted
    // All-or-None(AON) - should be done in a single execution
    // Immediate-or-cancel(IOC) - should be done instantly(single tick) or it gets deleted
     */
}

pub struct Order<OT: IsOrderType> {
    pub volume: f32,
    pub price: f32,
    pub timestamp: DateTime<FixedOffset>,
    order_type: PhantomData<fn() -> OT>,
}

impl<OT: IsOrderType> fmt::Debug for Order<OT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Order")
            .field("volume", &self.volume)
            .field("price", &self.price)
            .field("timestamp", &self.timestamp)
            .finish()
    }
}

impl<OT: IsOrderType> Order<OT> {
    fn new(volume: f32, price: f32) -> Self {
        Self {
            volume,
            price,
            order_type: PhantomData::<fn() -> OT>,
            timestamp: Local::now().into(),
        }
    }
}

impl<C> OrderBook<C> {
    pub fn with_commodity(commodity: C) -> Self {
        Self {
            commodity,
            sell_orders: vec![],
            buy_orders: vec![],
        }
    }

    pub fn add_buy(&mut self, volume: f32, price: f32) {
        self.buy_orders.push(Order::<Buy>::new(volume, price));
    }
    pub fn add_sell(&mut self, volume: f32, price: f32) {
        self.sell_orders.push(Order::<Sell>::new(volume, price));
    }
}

pub trait IsOrderType: fmt::Debug {}

#[derive(Debug)]
pub struct Buy;
impl IsOrderType for Buy {}

#[derive(Debug)]
pub struct Sell;
impl IsOrderType for Sell {}
