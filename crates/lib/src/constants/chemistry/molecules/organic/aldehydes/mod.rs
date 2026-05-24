use super::super::Molecule;

pub mod acetaldehyde;
pub mod benzaldehyde;
pub mod butyraldehyde;
pub mod cinnamaldehyde;
pub mod formaldehyde;
pub mod glyceraldehyde;
pub mod glyoxal;
pub mod propionaldehyde;
pub mod valeraldehyde;
pub mod vanillin;

pub fn all() -> Vec<Molecule> {
    vec![
        formaldehyde::molecule(),
        acetaldehyde::molecule(),
        propionaldehyde::molecule(),
        butyraldehyde::molecule(),
        valeraldehyde::molecule(),
        glyoxal::molecule(),
        glyceraldehyde::molecule(),
        benzaldehyde::molecule(),
        cinnamaldehyde::molecule(),
        vanillin::molecule(),
    ]
}
