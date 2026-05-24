pub mod dry_air;
pub mod nitrogen;
pub mod oxygen;
pub mod ozone;

pub use dry_air::*;
pub use nitrogen::*;
pub use oxygen::*;
pub use ozone::*;

use crate::moleculars::Gas;

pub fn all_atmosphere_gases() -> Vec<&'static Gas> {
    vec![
        &dry_air::DRY_AIR,
        &nitrogen::NITROGEN,
        &oxygen::OXYGEN,
        &ozone::OZONE,
    ]
}
