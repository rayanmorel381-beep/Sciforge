pub mod standard;

pub use standard::*;

use crate::lubrications::Grease;

pub fn all_polyurea_greases() -> Vec<&'static Grease> {
    vec![&standard::POLYUREA_GREASE]
}
