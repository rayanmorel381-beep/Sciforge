use crate::components::wheels::{AlloyRim, SteelRim, Rim, SummerTire, WinterTire, AllSeasonTire, Tire};

const BOLT_PATTERN: &str = "5x114.3";
const CENTER_BORE_MM: f64 = 60.1;

pub fn steel() -> Vec<Rim> {
    vec![
        Rim::Steel(SteelRim::stamped(16, 6.5, BOLT_PATTERN, CENTER_BORE_MM, 9.5)),
        Rim::Steel(SteelRim::rolled(16, 6.5, BOLT_PATTERN, CENTER_BORE_MM, 9.2)),
        Rim::Steel(SteelRim::stamped(17, 7.0, BOLT_PATTERN, CENTER_BORE_MM, 9.8)),
    ]
}

pub fn alloy() -> Vec<Rim> {
    vec![
        Rim::Alloy(AlloyRim::cast(16, 7.0, BOLT_PATTERN, CENTER_BORE_MM, 8.2)),
        Rim::Alloy(AlloyRim::cast(17, 7.5, BOLT_PATTERN, CENTER_BORE_MM, 8.5)),
        Rim::Alloy(AlloyRim::cast(18, 8.0, BOLT_PATTERN, CENTER_BORE_MM, 9.0)),
    ]
}

pub fn summer() -> Vec<Tire> {
    vec![
        Tire::Summer(SummerTire::new(205, 60, 16, 92, 'V')),
        Tire::Summer(SummerTire::new(215, 55, 17, 94, 'V')),
        Tire::Summer(SummerTire::new(225, 50, 17, 94, 'W')),
        Tire::Summer(SummerTire::new(225, 45, 18, 91, 'W')),
    ]
}

pub fn winter() -> Vec<Tire> {
    vec![
        Tire::Winter(WinterTire::studless(205, 60, 16, 92, 'H')),
        Tire::Winter(WinterTire::studless(215, 55, 17, 94, 'H')),
        Tire::Winter(WinterTire::studless(225, 50, 17, 94, 'H')),
    ]
}

pub fn all_season() -> Vec<Tire> {
    vec![
        Tire::AllSeason(AllSeasonTire::new(205, 60, 16, 92, 'V')),
        Tire::AllSeason(AllSeasonTire::new(215, 55, 17, 95, 'V')),
        Tire::AllSeason(AllSeasonTire::new(225, 50, 17, 95, 'W')),
        Tire::AllSeason(AllSeasonTire::new(225, 45, 18, 95, 'W')),
    ]
}
