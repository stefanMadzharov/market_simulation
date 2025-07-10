use rust_decimal::{dec, Decimal};

pub enum RiskAversion {
    VeryRiskAverse,
    RiskAverse,
    Normal,
    Risky,
    Reckless,
}

pub trait IsRiskAversion<Res = Decimal> {
    fn calculate(&self) -> Res;
}

impl IsRiskAversion for RiskAversion {
    fn calculate(&self) -> Decimal {
        match self {
            Self::VeryRiskAverse => Decimal::ONE,
            Self::RiskAverse => dec!(0.75),
            Self::Normal => dec!(0.5),
            Self::Risky => dec!(0.25),
            Self::Reckless => Decimal::ZERO,
        }
    }
}
