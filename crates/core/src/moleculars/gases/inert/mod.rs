pub mod argon;
pub mod helium;
pub mod krypton;
pub mod neon;
pub mod xenon;

pub use argon::*;
pub use helium::*;
pub use krypton::*;
pub use neon::*;
pub use xenon::*;

use crate::moleculars::Gas;

pub fn all_inert_gases() -> Vec<&'static Gas> {
    vec![
        &neon::NEON,
        &helium::HELIUM,
        &argon::ARGON,
        &krypton::KRYPTON,
        &xenon::XENON,
    ]
}
