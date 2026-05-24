use super::super::Molecule;

pub mod boric_acid;
pub mod carbonic_acid;
pub mod hydrobromic_acid;
pub mod hydrofluoric_acid;
pub mod hydroiodic_acid;
pub mod hypochlorous_acid;
pub mod nitrous_acid;
pub mod perchloric_acid;
pub mod phosphoric_acid;
pub mod sulfurous_acid;
pub mod chloric_acid;
pub mod hydrogen_chloride;
pub mod nitric_acid;
pub mod sulfuric_acid;

pub fn all() -> Vec<Molecule> {
    vec![
        hydrogen_chloride::molecule(),
        sulfuric_acid::molecule(),
        nitric_acid::molecule(),
        phosphoric_acid::molecule(),
        hydrofluoric_acid::molecule(),
        hydrobromic_acid::molecule(),
        hydroiodic_acid::molecule(),
        perchloric_acid::molecule(),
        carbonic_acid::molecule(),
        sulfurous_acid::molecule(),
        nitrous_acid::molecule(),
        boric_acid::molecule(),
        hypochlorous_acid::molecule(),
        chloric_acid::molecule(),
    ]
}
