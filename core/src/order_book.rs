use chrono::{DateTime, FixedOffset, Local};
use rust_decimal::Decimal;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt,
    marker::PhantomData,
};

#[derive(Default, Debug)]
pub struct OrderBook<C> {
    pub commodity: C,
    pub buy_orders: BTreeMap<Decimal, VecDeque<Order<Buy>>>,
    pub sell_orders: BTreeMap<Decimal, VecDeque<Order<Sell>>>,
    /*TODO
    // Add special types of orders:
    // Fill-or-Kill(FOK) - should be done instantly(single tick) and fully in a single execution or it gets deleted
    // All-or-None(AON) - should be done in a single execution
    // Immediate-or-cancel(IOC) - should be done instantly(single tick) or it gets deleted
     */
}

pub struct Order<OT: IsOrderType> {
    pub volume: f32,
    pub price: Decimal,
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
    fn new(volume: f32, price: Decimal) -> Self {
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
            sell_orders: BTreeMap::new(),
            buy_orders: BTreeMap::new(),
        }
    }

    pub fn add_buy(&mut self, volume: f32, price: Decimal) {
        self.buy_orders
            .entry(price)
            .and_modify(|queue| queue.push_back(Order::<Buy>::new(volume, price)))
            .or_insert_with(|| {
                let mut queue = VecDeque::new();
                queue.push_back(Order::<Buy>::new(volume, price));
                queue
            });
    }

    pub fn add_sell(&mut self, volume: f32, price: Decimal) {
        self.sell_orders
            .entry(price)
            .and_modify(|queue| queue.push_back(Order::<Sell>::new(volume, price)))
            .or_insert_with(|| {
                let mut queue = VecDeque::new();
                queue.push_back(Order::<Sell>::new(volume, price));
                queue
            });
    }
}

pub trait IsOrderType: fmt::Debug {}

#[derive(Debug)]
pub struct Buy;
impl IsOrderType for Buy {}

#[derive(Debug)]
pub struct Sell;
impl IsOrderType for Sell {}
