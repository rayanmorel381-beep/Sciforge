#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainSpeed {
    Speed6,
    Speed7,
    Speed8,
    Speed9,
    Speed10,
    Speed11,
    Speed12,
    Speed13,
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub speed: ChainSpeed,
    pub link_count: u16,
    pub hollow_pin: bool,
}

impl Chain {
    pub fn new(speed: ChainSpeed, link_count: u16) -> Self {
        Self { speed, link_count, hollow_pin: false }
    }

    pub fn lightweight(speed: ChainSpeed, link_count: u16) -> Self {
        Self { speed, link_count, hollow_pin: true }
    }
}
