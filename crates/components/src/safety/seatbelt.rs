#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatbeltType {
    TwoPoint,
    ThreePoint,
    FourPoint,
    FivePoint,
    SixPoint,
}

#[derive(Debug, Clone)]
pub struct Seatbelt {
    pub belt_type: SeatbeltType,
    pub pretensioner: bool,
    pub load_limiter: bool,
    pub height_adjustable: bool,
}

impl Seatbelt {
    pub fn three_point() -> Self {
        Self { belt_type: SeatbeltType::ThreePoint, pretensioner: true, load_limiter: true, height_adjustable: true }
    }

    pub fn harness(belt_type: SeatbeltType) -> Self {
        Self { belt_type, pretensioner: false, load_limiter: false, height_adjustable: false }
    }
}
