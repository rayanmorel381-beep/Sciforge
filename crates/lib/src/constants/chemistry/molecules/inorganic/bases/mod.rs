use super::super::Molecule;

pub mod aluminum_hydroxide;
pub mod ammonium_hydroxide;
pub mod barium_hydroxide;
pub mod calcium_hydroxide;
pub mod copper_hydroxide;
pub mod iron_hydroxide;
pub mod lithium_hydroxide;
pub mod magnesium_hydroxide;
pub mod potassium_hydroxide;
pub mod sodium_hydroxide;

pub fn all() -> Vec<Molecule> {
    vec![
        sodium_hydroxide::molecule(),
        potassium_hydroxide::molecule(),
        calcium_hydroxide::molecule(),
        magnesium_hydroxide::molecule(),
        aluminum_hydroxide::molecule(),
        ammonium_hydroxide::molecule(),
        lithium_hydroxide::molecule(),
        barium_hydroxide::molecule(),
        iron_hydroxide::molecule(),
        copper_hydroxide::molecule(),
    ]
}
