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

pub fn comfort() -> SeatKit {
    SeatKit {
        front_left: ComfortSeat::standard(),
        front_right: ComfortSeat::standard(),
        rear: BenchSeat { config: BenchConfig::Split60_40, foldable: true, heated: false },
    }
}

pub fn premium() -> SeatKit {
    SeatKit {
        front_left: ComfortSeat::executive(),
        front_right: ComfortSeat::executive(),
        rear: BenchSeat { config: BenchConfig::Split60_40, foldable: true, heated: true },
    }
}

pub fn all() -> Vec<SeatKit> {
    vec![entry(), comfort(), premium()]
}
