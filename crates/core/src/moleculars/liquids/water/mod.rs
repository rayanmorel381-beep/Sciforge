pub mod brine;
pub mod deionized_water;
pub mod distilled_water;
pub mod fresh_water;
pub mod seawater;

pub use brine::*;
pub use deionized_water::*;
pub use distilled_water::*;
pub use fresh_water::*;
pub use seawater::*;

use crate::moleculars::Liquid;

pub fn all_water() -> Vec<&'static Liquid> {
    vec![&WATER, &SEAWATER, &DISTILLED_WATER, &DEIONIZED_WATER, &BRINE]
}
