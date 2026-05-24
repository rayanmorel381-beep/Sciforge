pub mod mos2;

pub use mos2::*;

use crate::lubrications::Grease;

pub fn all_molybdenum_greases() -> Vec<&'static Grease> {
    vec![&mos2::MOLYBDENUM_GREASE]
}
