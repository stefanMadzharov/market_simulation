use rust_decimal::{dec, Decimal};

pub trait IsSentiment<Res = Decimal> {
    fn calculate(&self) -> Res;
}

pub enum Sentiment {
    VeryOptimistic,
    Optimistic,
    Normal,
    Pessimistic,
    VeryPessimistic,
}

impl IsSentiment for Sentiment {
    fn calculate(&self) -> Decimal {
        match self {
            Self::VeryOptimistic => Decimal::ONE,
            Self::Optimistic => dec!(0.75),
            Self::Normal => dec!(0.5),
            Self::Pessimistic => dec!(0.25),
            Self::VeryPessimistic => Decimal::ZERO,
        }
    }
}
