use super::super::Molecule;

pub mod acetic_acid;
pub mod adipic_acid;
pub mod benzoic_acid;
pub mod butyric_acid;
pub mod citric_acid;
pub mod formic_acid;
pub mod glutaric_acid;
pub mod lactic_acid;
pub mod malic_acid;
pub mod malonic_acid;
pub mod oxalic_acid;
pub mod propionic_acid;
pub mod pyruvic_acid;
pub mod salicylic_acid;
pub mod succinic_acid;
pub mod tartaric_acid;
pub mod valeric_acid;

pub fn all() -> Vec<Molecule> {
    vec![
        formic_acid::molecule(),
        acetic_acid::molecule(),
        propionic_acid::molecule(),
        butyric_acid::molecule(),
        valeric_acid::molecule(),
        oxalic_acid::molecule(),
        malonic_acid::molecule(),
        succinic_acid::molecule(),
        glutaric_acid::molecule(),
        adipic_acid::molecule(),
        lactic_acid::molecule(),
        pyruvic_acid::molecule(),
        malic_acid::molecule(),
        tartaric_acid::molecule(),
        citric_acid::molecule(),
        benzoic_acid::molecule(),
        salicylic_acid::molecule(),
    ]
}
