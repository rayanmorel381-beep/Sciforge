use crate::constants::chemistry::molecules::Molecule;

pub mod dry_air;
pub mod wet_air;

pub fn all() -> Vec<Molecule> {
    vec![dry_air::molecule(), wet_air::molecule()]
}
