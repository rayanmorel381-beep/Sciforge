pub mod antiseize;
pub mod brake_paste;
pub mod exhaust_paste;

pub use antiseize::*;
pub use brake_paste::*;
pub use exhaust_paste::*;

use crate::lubrications::Grease;

pub fn all_copper_greases() -> Vec<&'static Grease> {
    vec![
        &antiseize::COPPER_ANTISEIZE,
        &brake_paste::COPPER_BRAKE_PASTE,
        &exhaust_paste::COPPER_EXHAUST_PASTE,
    ]
}
