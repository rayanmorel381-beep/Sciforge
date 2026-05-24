use super::super::Molecule;

pub mod albite;
pub mod almandine;
pub mod anorthite;
pub mod azurite;
pub mod beryl;
pub mod diopside;
pub mod dolomite;
pub mod enstatite;
pub mod fayalite;
pub mod forsterite;
pub mod kaolinite;
pub mod malachite;
pub mod montmorillonite;
pub mod muscovite;
pub mod orthoclase;
pub mod rhodochrosite;
pub mod serpentine;
pub mod siderite;
pub mod talc;
pub mod tremolite;
pub mod zircon;

pub fn all() -> Vec<Molecule> {
    vec![
        kaolinite::molecule(),
        montmorillonite::molecule(),
        muscovite::molecule(),
        orthoclase::molecule(),
        albite::molecule(),
        anorthite::molecule(),
        forsterite::molecule(),
        fayalite::molecule(),
        enstatite::molecule(),
        diopside::molecule(),
        tremolite::molecule(),
        almandine::molecule(),
        zircon::molecule(),
        talc::molecule(),
        serpentine::molecule(),
        beryl::molecule(),
        dolomite::molecule(),
        siderite::molecule(),
        rhodochrosite::molecule(),
        malachite::molecule(),
        azurite::molecule(),
    ]
}
