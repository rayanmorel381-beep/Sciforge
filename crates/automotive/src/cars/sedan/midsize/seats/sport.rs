use crate::components::seats::{BenchConfig, BenchSeat, SportSeat};

#[derive(Debug, Clone)]
pub struct SeatKit {
    pub front_left: SportSeat,
    pub front_right: SportSeat,
    pub rear: BenchSeat,
}

pub fn sport() -> SeatKit {
    SeatKit {
        front_left: SportSeat::electric(true, true),
        front_right: SportSeat::electric(true, true),
        rear: BenchSeat::foldable(BenchConfig::Split60_40),
    }
}

pub fn all() -> Vec<SeatKit> {
    vec![sport()]
}
