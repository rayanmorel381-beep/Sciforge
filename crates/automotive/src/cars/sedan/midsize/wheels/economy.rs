use crate::components::wheels::{AlloyRim, SteelRim, Rim, SummerTire, WinterTire, AllSeasonTire, Tire};

const BOLT_PATTERN: &str = "5x112";
const CENTER_BORE_MM: f64 = 57.1;

pub fn steel() -> Vec<Rim> {
    vec![
        Rim::Steel(SteelRim::stamped(15, 6.0, BOLT_PATTERN, CENTER_BORE_MM, 9.2)),
        Rim::Steel(SteelRim::stamped(16, 6.5, BOLT_PATTERN, CENTER_BORE_MM, 9.5)),
    ]
}

pub fn alloy() -> Vec<Rim> {
    vec![
        Rim::Alloy(AlloyRim::cast(15, 6.0, BOLT_PATTERN, CENTER_BORE_MM, 7.8)),
        Rim::Alloy(AlloyRim::cast(16, 6.5, BOLT_PATTERN, CENTER_BORE_MM, 8.2)),
    ]
}

pub fn summer() -> Vec<Tire> {
    vec![
        Tire::Summer(SummerTire::new(185, 65, 15, 88, 'H')),
        Tire::Summer(SummerTire::new(195, 60, 15, 88, 'V')),
        Tire::Summer(SummerTire::new(205, 55, 16, 91, 'V')),
    ]
}

pub fn winter() -> Vec<Tire> {
    vec![
        Tire::Winter(WinterTire::studless(185, 65, 15, 88, 'H')),
        Tire::Winter(WinterTire::studless(195, 60, 15, 88, 'H')),
        Tire::Winter(WinterTire::studded(185, 65, 15, 88, 'H')),
    ]
}

pub fn all_season() -> Vec<Tire> {
    vec![
        Tire::AllSeason(AllSeasonTire::new(185, 65, 15, 88, 'H')),
        Tire::AllSeason(AllSeasonTire::new(195, 60, 15, 88, 'V')),
        Tire::AllSeason(AllSeasonTire::new(205, 55, 16, 91, 'V')),
    ]
}
