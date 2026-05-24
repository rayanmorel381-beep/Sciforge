pub mod aluminum_complex;

pub use aluminum_complex::*;

use crate::lubrications::Grease;

pub fn all_aluminum_greases() -> Vec<&'static Grease> {
    vec![&aluminum_complex::ALUMINUM_COMPLEX_GREASE]
}
