use super::super::Molecule;

pub mod aluminum_oxide;
pub mod iron_oxide;
pub mod magnesium_oxide;
pub mod silicon_dioxide;
pub mod titanium_dioxide;

pub fn all() -> Vec<Molecule> {
    vec![
        silicon_dioxide::molecule(),
        iron_oxide::molecule(),
        aluminum_oxide::molecule(),
        magnesium_oxide::molecule(),
        titanium_dioxide::molecule(),
    ]
}
