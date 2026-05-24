pub mod ethylene_glycol_water_50;
pub mod propylene_glycol_water_50;
pub mod waterless_coolant;

pub use ethylene_glycol_water_50::*;
pub use propylene_glycol_water_50::*;
pub use waterless_coolant::*;

use crate::moleculars::Liquid;

pub fn all_coolants() -> Vec<&'static Liquid> {
    vec![
        &ETHYLENE_GLYCOL_WATER_50,
        &PROPYLENE_GLYCOL_WATER_50,
        &WATERLESS_COOLANT,
    ]
}
