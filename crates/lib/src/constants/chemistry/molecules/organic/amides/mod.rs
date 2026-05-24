use super::super::Molecule;

pub mod acetamide;
pub mod benzamide;
pub mod dimethylacetamide;
pub mod dimethylformamide;
pub mod formamide;
pub mod n_methylacetamide;

pub fn all() -> Vec<Molecule> {
    vec![
        formamide::molecule(),
        acetamide::molecule(),
        n_methylacetamide::molecule(),
        dimethylformamide::molecule(),
        dimethylacetamide::molecule(),
        benzamide::molecule(),
    ]
}
