pub mod petroleum;
pub mod vegetable;

pub use petroleum::*;
pub use vegetable::*;

use crate::moleculars::Liquid;

pub fn all_oils() -> Vec<&'static Liquid> {
    let mut oils = Vec::new();
    oils.extend(vegetable::all_vegetable_oils());
    oils.extend(petroleum::all_petroleum_oils());
    oils
}
