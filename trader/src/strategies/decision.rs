use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Position {
    pub price: Decimal,
    pub volume: Decimal,
}

#[derive(Debug)]
pub enum Decision {
    Buying { position: Position },
    Holding,
    Selling { position: Position },
}
