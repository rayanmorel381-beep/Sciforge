pub mod atf;
pub mod hydraulic_oil_iso32;
pub mod hydraulic_oil_iso46;
pub mod phosphate_ester;

pub use atf::*;
pub use hydraulic_oil_iso32::*;
pub use hydraulic_oil_iso46::*;
pub use phosphate_ester::*;

use crate::moleculars::Liquid;

pub fn all_hydraulics() -> Vec<&'static Liquid> {
    vec![
        &HYDRAULIC_OIL_ISO32,
        &HYDRAULIC_OIL_ISO46,
        &ATF,
        &PHOSPHATE_ESTER_HFD,
    ]
}
