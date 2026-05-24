use super::super::Molecule;

pub mod dichlorine;
pub mod difluorine;
pub mod dihydrogen;
pub mod dinitrogen;
pub mod dioxygen;
pub mod ozone;

pub fn all() -> Vec<Molecule> {
    vec![
        dihydrogen::molecule(),
        dinitrogen::molecule(),
        dioxygen::molecule(),
        ozone::molecule(),
        dichlorine::molecule(),
        difluorine::molecule(),
    ]
}
