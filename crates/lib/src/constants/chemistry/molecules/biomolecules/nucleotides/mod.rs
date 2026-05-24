use super::super::Molecule;

pub mod adp;
pub mod amp;
pub mod atp;
pub mod cmp;
pub mod coa;
pub mod fad;
pub mod gdp;
pub mod gmp;
pub mod gtp;
pub mod nad;
pub mod nadp;
pub mod ump;

pub fn all() -> Vec<Molecule> {
    vec![
        amp::molecule(),
        adp::molecule(),
        atp::molecule(),
        gmp::molecule(),
        gdp::molecule(),
        gtp::molecule(),
        cmp::molecule(),
        ump::molecule(),
        nad::molecule(),
        nadp::molecule(),
        fad::molecule(),
        coa::molecule(),
    ]
}
