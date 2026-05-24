use crate::components::wheels::{AlloyRim, Rim, SummerTire, Tire};

const BOLT_PATTERN: &str = "5x112";
const CENTER_BORE_MM: f64 = 57.1;

pub fn sport_rims() -> Vec<Rim> {
    vec![
        Rim::Alloy(AlloyRim::forged(18, 8.5, BOLT_PATTERN, CENTER_BORE_MM, 8.2)),
        Rim::Alloy(AlloyRim::forged(19, 9.0, BOLT_PATTERN, CENTER_BORE_MM, 8.8)),
        Rim::Alloy(AlloyRim::forged(20, 9.5, BOLT_PATTERN, CENTER_BORE_MM, 9.5)),
    ]
}

pub fn sport_tires() -> Vec<Tire> {
    vec![
        Tire::Summer(SummerTire::new(225, 45, 18, 91, 'Y')),
        Tire::Summer(SummerTire::new(235, 40, 19, 92, 'Y')),
        Tire::Summer(SummerTire::new(245, 35, 20, 91, 'Y')),
        Tire::Summer(SummerTire::run_flat(225, 45, 18, 91, 'Y')),
        Tire::Summer(SummerTire::run_flat(235, 40, 19, 92, 'Y')),
    ]
}
