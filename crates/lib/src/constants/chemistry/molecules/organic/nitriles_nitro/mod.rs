use super::super::Molecule;

pub mod acetonitrile;
pub mod benzonitrile;
pub mod nitrobenzene;
pub mod nitromethane;
pub mod propionitrile;
pub mod trinitrotoluene;

pub fn all() -> Vec<Molecule> {
    vec![
        acetonitrile::molecule(),
        propionitrile::molecule(),
        benzonitrile::molecule(),
        nitromethane::molecule(),
        nitrobenzene::molecule(),
        trinitrotoluene::molecule(),
    ]
}
