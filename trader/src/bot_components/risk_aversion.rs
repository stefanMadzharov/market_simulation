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
            Self::VeryRiskAverse => dec!(0.9),
            Self::RiskAverse => dec!(0.2),
            Self::Normal => dec!(0),
            Self::Risky => dec!(-0.2),
            Self::Reckless => dec!(-0.9),
        }
    }
}
