#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatAdjust {
    Manual,
    Electric,
    ElectricWithMemory,
}

#[derive(Debug, Clone)]
pub struct SportSeat {
    pub adjust: SeatAdjust,
    pub side_bolster_height_mm: f64,
    pub harness_compatible: bool,
    pub heated: bool,
    pub ventilated: bool,
}

impl SportSeat {
    pub fn standard() -> Self {
        Self { adjust: SeatAdjust::Manual, side_bolster_height_mm: 80.0, harness_compatible: false, heated: false, ventilated: false }
    }

    pub fn race(harness_compatible: bool) -> Self {
        Self { adjust: SeatAdjust::Manual, side_bolster_height_mm: 120.0, harness_compatible, heated: false, ventilated: false }
    }

    pub fn electric(heated: bool, ventilated: bool) -> Self {
        Self { adjust: SeatAdjust::ElectricWithMemory, side_bolster_height_mm: 90.0, harness_compatible: false, heated, ventilated }
    }
}
