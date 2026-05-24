pub mod calcium_sulfonate;
pub mod conventional;

pub use calcium_sulfonate::*;
pub use conventional::*;

use crate::lubrications::Grease;

pub fn all_calcium_greases() -> Vec<&'static Grease> {
    vec![
        &conventional::CALCIUM_GREASE,
        &calcium_sulfonate::CALCIUM_SULFONATE_GREASE,
    ]
}
