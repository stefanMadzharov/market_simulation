use core::structs::market::IsCommodity;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct Trader<C: IsCommodity> {
    pub balance: Decimal,
    pub commodities_volume: HashMap<C, Decimal>,
}

impl<C: IsCommodity> Trader<C> {
    pub fn calculate_net_worth(&self, prices: &HashMap<C, Decimal>) -> Decimal {
        self.balance + self.calculate_commodities_worth(prices)
    }

    pub fn calculate_commodities_worth(&self, prices: &HashMap<C, Decimal>) -> Decimal {
        self.commodities_volume
            .iter()
            .filter_map(|(comm, volume)| prices.get(comm).map(|comm| (comm, volume)))
            .map(|(comm, volume)| comm * volume)
            .sum()
    }
}
