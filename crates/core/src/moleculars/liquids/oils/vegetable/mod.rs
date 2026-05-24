pub mod canola_oil;
pub mod castor_oil;
pub mod corn_oil;
pub mod olive_oil;
pub mod palm_oil;
pub mod soybean_oil;
pub mod sunflower_oil;
pub mod vegetable_oil;

pub use canola_oil::*;
pub use castor_oil::*;
pub use corn_oil::*;
pub use olive_oil::*;
pub use palm_oil::*;
pub use soybean_oil::*;
pub use sunflower_oil::*;
pub use vegetable_oil::*;

use crate::moleculars::Liquid;

pub fn all_vegetable_oils() -> Vec<&'static Liquid> {
    vec![
        &VEGETABLE_OIL,
        &CANOLA_OIL,
        &SOYBEAN_OIL,
        &SUNFLOWER_OIL,
        &OLIVE_OIL,
        &PALM_OIL,
        &CORN_OIL,
        &CASTOR_OIL,
    ]
}
