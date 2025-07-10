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
            Self::VeryOptimistic => dec!(0.9),
            Self::Optimistic => dec!(0.2),
            Self::Normal => dec!(0),
            Self::Pessimistic => dec!(-0.2),
            Self::VeryPessimistic => dec!(-0.9),
        }
    }
}
