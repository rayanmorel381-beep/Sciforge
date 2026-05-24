use super::super::Molecule;

pub mod cyclobutane;
pub mod cycloheptane;
pub mod cyclohexane;
pub mod cyclooctane;
pub mod cyclopentane;
pub mod cyclopropane;
pub mod decalin;
pub mod methylcyclohexane;

pub fn all() -> Vec<Molecule> {
    vec![
        cyclopropane::molecule(),
        cyclobutane::molecule(),
        cyclopentane::molecule(),
        cyclohexane::molecule(),
        cycloheptane::molecule(),
        cyclooctane::molecule(),
        methylcyclohexane::molecule(),
        decalin::molecule(),
    ]
}
