pub mod chlorine;
pub mod fluorine;
pub mod hydrogen_sulfide;
pub mod nitrous_oxide;
pub mod sulfur_hexafluoride;

pub use chlorine::*;
pub use fluorine::*;
pub use hydrogen_sulfide::*;
pub use nitrous_oxide::*;
pub use sulfur_hexafluoride::*;

use crate::moleculars::Gas;

pub fn all_industrial_gases() -> Vec<&'static Gas> {
    vec![
        &chlorine::CHLORINE,
        &sulfur_hexafluoride::SULFUR_HEXAFLUORIDE,
        &nitrous_oxide::NITROUS_OXIDE,
        &fluorine::FLUORINE,
        &hydrogen_sulfide::HYDROGEN_SULFIDE,
    ]
}
