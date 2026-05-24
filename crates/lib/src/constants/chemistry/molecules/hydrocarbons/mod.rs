use super::Molecule;

pub mod alkanes;
pub mod alkenes;
pub mod alkynes;
pub mod aromatics;
pub mod cycloalkanes;

pub fn all() -> Vec<Molecule> {
    let mut v = Vec::new();
    v.extend(alkanes::all());
    v.extend(alkenes::all());
    v.extend(alkynes::all());
    v.extend(cycloalkanes::all());
    v.extend(aromatics::all());
    v
}
