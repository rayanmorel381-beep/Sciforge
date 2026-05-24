use super::super::Molecule;

pub mod bromomethane;
pub mod carbon_tetrachloride;
pub mod chlorobenzene;
pub mod chloroethane;
pub mod chloroform;
pub mod chloromethane;
pub mod dichloromethane;
pub mod hfc_134a;
pub mod hfc_125;
pub mod hfc_32;
pub mod hfo_1234yf;
pub mod iodomethane;
pub mod perchloroethylene;
pub mod r12;
pub mod r410a;
pub mod tetrafluoromethane;
pub mod trichloroethylene;
pub mod vinyl_bromide;
pub mod vinyl_chloride;

pub fn all() -> Vec<Molecule> {
    vec![
        chloromethane::molecule(),
        dichloromethane::molecule(),
        chloroform::molecule(),
        carbon_tetrachloride::molecule(),
        bromomethane::molecule(),
        iodomethane::molecule(),
        chloroethane::molecule(),
        vinyl_chloride::molecule(),
        vinyl_bromide::molecule(),
        chlorobenzene::molecule(),
        trichloroethylene::molecule(),
        perchloroethylene::molecule(),
        r12::molecule(),
        hfc_134a::molecule(),
        hfc_32::molecule(),
        hfc_125::molecule(),
        hfo_1234yf::molecule(),
        r410a::molecule(),
        tetrafluoromethane::molecule(),
    ]
}
