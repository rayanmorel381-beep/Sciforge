pub mod carbon_dioxide;
pub mod carbon_monoxide;
pub mod nitric_oxide;
pub mod nitrogen_dioxide;
pub mod sulfur_dioxide;
pub mod water_vapor;

pub use carbon_dioxide::*;
pub use carbon_monoxide::*;
pub use nitric_oxide::*;
pub use nitrogen_dioxide::*;
pub use sulfur_dioxide::*;
pub use water_vapor::*;

use crate::moleculars::Gas;

pub fn all_combustion_gases() -> Vec<&'static Gas> {
    vec![
        &carbon_dioxide::CARBON_DIOXIDE,
        &carbon_monoxide::CARBON_MONOXIDE,
        &water_vapor::WATER_VAPOR,
        &sulfur_dioxide::SULFUR_DIOXIDE,
        &nitric_oxide::NITRIC_OXIDE,
        &nitrogen_dioxide::NITROGEN_DIOXIDE,
    ]
}
