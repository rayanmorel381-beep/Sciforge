use super::super::Molecule;

pub mod butane;
pub mod decane;
pub mod dimethylbutane_2_2;
pub mod dimethylbutane_2_3;
pub mod dodecane;
pub mod ethane;
pub mod heptadecane;
pub mod heptane;
pub mod hexadecane;
pub mod hexane;
pub mod icosane;
pub mod isobutane;
pub mod isohexane;
pub mod isooctane;
pub mod isopentane;
pub mod methane;
pub mod methylpentane_3;
pub mod neopentane;
pub mod nonadecane;
pub mod nonane;
pub mod octadecane;
pub mod octane;
pub mod pentadecane;
pub mod pentane;
pub mod propane;
pub mod tetradecane;
pub mod tridecane;
pub mod undecane;

pub fn all() -> Vec<Molecule> {
    vec![
        methane::molecule(),
        ethane::molecule(),
        propane::molecule(),
        butane::molecule(),
        pentane::molecule(),
        hexane::molecule(),
        heptane::molecule(),
        octane::molecule(),
        nonane::molecule(),
        decane::molecule(),
        undecane::molecule(),
        dodecane::molecule(),
        tridecane::molecule(),
        tetradecane::molecule(),
        pentadecane::molecule(),
        hexadecane::molecule(),
        heptadecane::molecule(),
        octadecane::molecule(),
        nonadecane::molecule(),
        icosane::molecule(),
        isobutane::molecule(),
        isopentane::molecule(),
        neopentane::molecule(),
        isohexane::molecule(),
        methylpentane_3::molecule(),
        dimethylbutane_2_2::molecule(),
        dimethylbutane_2_3::molecule(),
        isooctane::molecule(),
    ]
}
