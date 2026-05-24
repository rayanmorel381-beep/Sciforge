use super::Molecule;

pub mod amino_acids;
pub mod lipids;
pub mod nucleobases;
pub mod nucleotides;
pub mod signaling;
pub mod sugars;
pub mod urea;
pub mod vitamins;

pub fn all() -> Vec<Molecule> {
    let mut v = vec![urea::molecule()];
    v.extend(amino_acids::all());
    v.extend(nucleobases::all());
    v.extend(nucleotides::all());
    v.extend(sugars::all());
    v.extend(lipids::all());
    v.extend(vitamins::all());
    v.extend(signaling::all());
    v
}
