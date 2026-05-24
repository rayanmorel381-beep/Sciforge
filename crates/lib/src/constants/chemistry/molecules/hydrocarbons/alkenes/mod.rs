use super::super::Molecule;

pub mod butadiene_1_3;
pub mod butene_1;
pub mod decene_1;
pub mod ethene;
pub mod heptene_1;
pub mod hexene_1;
pub mod isobutene;
pub mod isoprene;
pub mod nonene_1;
pub mod octene_1;
pub mod pentene_1;
pub mod propene;

pub fn all() -> Vec<Molecule> {
    vec![
        ethene::molecule(),
        propene::molecule(),
        butene_1::molecule(),
        pentene_1::molecule(),
        hexene_1::molecule(),
        heptene_1::molecule(),
        octene_1::molecule(),
        nonene_1::molecule(),
        decene_1::molecule(),
        isobutene::molecule(),
        butadiene_1_3::molecule(),
        isoprene::molecule(),
    ]
}
