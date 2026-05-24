use super::super::Molecule;

pub mod anisole;
pub mod diethyl_ether;
pub mod dimethyl_ether;
pub mod dioxane_1_4;
pub mod methyl_tert_butyl_ether;
pub mod tetrahydrofuran;

pub fn all() -> Vec<Molecule> {
    vec![
        dimethyl_ether::molecule(),
        diethyl_ether::molecule(),
        methyl_tert_butyl_ether::molecule(),
        tetrahydrofuran::molecule(),
        dioxane_1_4::molecule(),
        anisole::molecule(),
    ]
}
