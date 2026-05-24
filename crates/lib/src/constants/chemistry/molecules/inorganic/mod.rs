use super::Molecule;

pub mod acids;
pub mod air;
pub mod bases;
pub mod elements;
pub mod hydrides_etc;
pub mod oxides;
pub mod salts;

pub fn all() -> Vec<Molecule> {
    let mut v = Vec::new();
    v.extend(air::all());
    v.extend(elements::all());
    v.extend(acids::all());
    v.extend(bases::all());
    v.extend(salts::all());
    v.extend(oxides::all());
    v.extend(hydrides_etc::all());
    v
}
