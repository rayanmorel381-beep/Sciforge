pub mod lithium_complex;
pub mod simple;

pub use lithium_complex::*;
pub use simple::*;

use crate::lubrications::Grease;

pub fn all_lithium_greases() -> Vec<&'static Grease> {
    vec![
        &simple::LITHIUM_GREASE,
        &lithium_complex::LITHIUM_COMPLEX_GREASE,
    ]
}
