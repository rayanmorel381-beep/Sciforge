#[derive(Debug, Clone)]
pub struct BrakeBias {
    pub front_pct: u8,
    pub rear_pct: u8,
    pub adjustable: bool,
    pub brake_balance_bar: bool,
}

impl BrakeBias {
    pub fn road(front_pct: u8) -> Self {
        Self { front_pct, rear_pct: 100 - front_pct, adjustable: false, brake_balance_bar: false }
    }

    pub fn racing(front_pct: u8) -> Self {
        Self { front_pct, rear_pct: 100 - front_pct, adjustable: true, brake_balance_bar: true }
    }
}
