use super::Molecule;

pub mod oxides;
pub mod silicates_carbonates;
pub mod sulfides_etc;

pub fn all() -> Vec<Molecule> {
    let mut v = Vec::new();
    v.extend(oxides::all());
    v.extend(silicates_carbonates::all());
    v.extend(sulfides_etc::all());
    v
}
