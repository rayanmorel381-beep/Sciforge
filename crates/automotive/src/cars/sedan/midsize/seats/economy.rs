use crate::components::seats::{BenchConfig, BenchSeat, ComfortSeat};

#[derive(Debug, Clone)]
pub struct SeatKit {
    pub front_left: ComfortSeat,
    pub front_right: ComfortSeat,
    pub rear: BenchSeat,
}

pub fn entry() -> SeatKit {
    SeatKit {
        front_left: ComfortSeat { features: vec![], memory_positions: 0, lumbar_support: false, extendable_thigh: false },
        front_right: ComfortSeat { features: vec![], memory_positions: 0, lumbar_support: false, extendable_thigh: false },
        rear: BenchSeat::foldable(BenchConfig::Split60_40),
    }
}

pub fn all() -> Vec<SeatKit> {
    vec![entry()]
}
