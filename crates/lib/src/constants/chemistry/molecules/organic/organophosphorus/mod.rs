use super::super::Molecule;

pub mod dimethyl_methylphosphonate;
pub mod glyphosate;
pub mod malathion;
pub mod trimethyl_phosphate;
pub mod triethyl_phosphate;

pub fn all() -> Vec<Molecule> {
    vec![
        trimethyl_phosphate::molecule(),
        triethyl_phosphate::molecule(),
        dimethyl_methylphosphonate::molecule(),
        glyphosate::molecule(),
        malathion::molecule(),
    ]
}
