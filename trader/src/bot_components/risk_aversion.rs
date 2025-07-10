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
            Self::RiskAverse => dec!(0.5),

            Self::Normal => Decimal::ZERO,

            Self::Risky => dec!(-0.5),
            Self::Reckless => -Decimal::ONE,
        }
    }
}
