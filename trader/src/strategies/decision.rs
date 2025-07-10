use rust_decimal::Decimal;

pub struct Position {
    pub price: Decimal,
    pub volume: f32,
}

pub enum Decision {
    Buying { position: Position },
    Holding,
    Selling { position: Position },
}
