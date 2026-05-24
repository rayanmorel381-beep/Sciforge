pub mod simple;

pub use simple::*;

use crate::lubrications::Grease;

pub fn all_sodium_greases() -> Vec<&'static Grease> {
    vec![&simple::SODIUM_GREASE]
}
