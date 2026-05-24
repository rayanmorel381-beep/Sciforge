use super::super::Molecule;

pub mod butyne_1;
pub mod butyne_2;
pub mod ethyne;
pub mod heptyne_1;
pub mod hexyne_1;
pub mod octyne_1;
pub mod pentyne_1;
pub mod propyne;

pub fn all() -> Vec<Molecule> {
    vec![
        ethyne::molecule(),
        propyne::molecule(),
        butyne_1::molecule(),
        butyne_2::molecule(),
        pentyne_1::molecule(),
        hexyne_1::molecule(),
        heptyne_1::molecule(),
        octyne_1::molecule(),
    ]
}
