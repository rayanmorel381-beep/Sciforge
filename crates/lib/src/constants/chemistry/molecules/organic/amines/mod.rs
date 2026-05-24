use super::super::Molecule;

pub mod aniline;
pub mod diethylamine;
pub mod dimethylamine;
pub mod ethanolamine;
pub mod ethylamine;
pub mod ethylenediamine;
pub mod methylamine;
pub mod propylamine;
pub mod triethylamine;
pub mod trimethylamine;

pub fn all() -> Vec<Molecule> {
    vec![
        methylamine::molecule(),
        dimethylamine::molecule(),
        trimethylamine::molecule(),
        ethylamine::molecule(),
        diethylamine::molecule(),
        triethylamine::molecule(),
        propylamine::molecule(),
        ethanolamine::molecule(),
        ethylenediamine::molecule(),
        aniline::molecule(),
    ]
}
