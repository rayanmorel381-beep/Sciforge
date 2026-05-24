#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorPosition {
    Front,
    Side,
    Rear,
    Mirror,
}

#[derive(Debug, Clone)]
pub struct Indicator {
    pub position: IndicatorPosition,
    pub led: bool,
    pub sequential: bool,
}

impl Indicator {
    pub fn standard(position: IndicatorPosition) -> Self {
        Self { position, led: false, sequential: false }
    }

    pub fn led(position: IndicatorPosition) -> Self {
        Self { position, led: true, sequential: false }
    }

    pub fn sequential(position: IndicatorPosition) -> Self {
        Self { position, led: true, sequential: true }
    }
}
