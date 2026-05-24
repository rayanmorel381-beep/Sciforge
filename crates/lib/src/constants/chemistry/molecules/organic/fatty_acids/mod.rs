use super::super::Molecule;

pub mod arachidonic_acid;
pub mod capric_acid;
pub mod caproic_acid;
pub mod caprylic_acid;
pub mod lauric_acid;
pub mod linoleic_acid;
pub mod linolenic_acid;
pub mod myristic_acid;
pub mod oleic_acid;
pub mod palmitic_acid;
pub mod stearic_acid;

pub fn all() -> Vec<Molecule> {
    vec![
        caproic_acid::molecule(),
        caprylic_acid::molecule(),
        capric_acid::molecule(),
        lauric_acid::molecule(),
        myristic_acid::molecule(),
        palmitic_acid::molecule(),
        stearic_acid::molecule(),
        oleic_acid::molecule(),
        linoleic_acid::molecule(),
        linolenic_acid::molecule(),
        arachidonic_acid::molecule(),
    ]
}
