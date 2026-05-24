use super::super::Molecule;

pub mod furan;
pub mod imidazole;
pub mod oxazole;
pub mod pyrazole;
pub mod pyridine;
pub mod pyrimidine;
pub mod pyrrole;
pub mod thiazole;

pub fn all() -> Vec<Molecule> {
    vec![
        furan::molecule(),
        pyrrole::molecule(),
        imidazole::molecule(),
        pyrazole::molecule(),
        oxazole::molecule(),
        thiazole::molecule(),
        pyridine::molecule(),
        pyrimidine::molecule(),
    ]
}
