pub mod r134a;
pub mod r32;
pub mod r410a;
pub mod r717;
pub mod r744;

pub use r134a::*;
pub use r32::*;
pub use r410a::*;
pub use r717::*;
pub use r744::*;

use crate::moleculars::Gas;

pub fn all_refrigerant_gases() -> Vec<&'static Gas> {
    vec![
        &r134a::R134A,
        &r32::R32,
        &r410a::R410A,
        &r744::R744,
        &r717::R717,
    ]
}
