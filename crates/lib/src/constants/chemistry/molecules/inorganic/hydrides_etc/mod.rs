use super::super::Molecule;

pub mod aluminum_hydride;
pub mod borane;
pub mod calcium_hydride;
pub mod diborane;
pub mod hydrogen_peroxide;
pub mod lithium_hydride;
pub mod magnesium_hydride;
pub mod potassium_ferricyanide;
pub mod potassium_hydride;
pub mod potassium_superoxide;
pub mod sodium_hydride;
pub mod sodium_peroxide;
pub mod ammonia;
pub mod hydrogen_sulfide;
pub mod sulfur_hexafluoride;

pub fn all() -> Vec<Molecule> {
    vec![
        ammonia::molecule(),
        hydrogen_sulfide::molecule(),
        sulfur_hexafluoride::molecule(),
        lithium_hydride::molecule(),
        sodium_hydride::molecule(),
        potassium_hydride::molecule(),
        magnesium_hydride::molecule(),
        calcium_hydride::molecule(),
        aluminum_hydride::molecule(),
        borane::molecule(),
        diborane::molecule(),
        hydrogen_peroxide::molecule(),
        sodium_peroxide::molecule(),
        potassium_superoxide::molecule(),
        potassium_ferricyanide::molecule(),
    ]
}
