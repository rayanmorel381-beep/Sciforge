pub mod acetylene;
pub mod ammonia;
pub mod butane;
pub mod ethane;
pub mod ethylene;
pub mod hydrogen;
pub mod methane;
pub mod propane;

pub use acetylene::*;
pub use ammonia::*;
pub use butane::*;
pub use ethane::*;
pub use ethylene::*;
pub use hydrogen::*;
pub use methane::*;
pub use propane::*;

use crate::moleculars::Gas;

pub fn all_fuel_gases() -> Vec<&'static Gas> {
    vec![
        &hydrogen::HYDROGEN,
        &methane::METHANE,
        &ethane::ETHANE,
        &propane::PROPANE,
        &butane::BUTANE,
        &ethylene::ETHYLENE,
        &acetylene::ACETYLENE,
        &ammonia::AMMONIA,
    ]
}
