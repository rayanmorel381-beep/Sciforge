use super::super::Molecule;

pub mod apatite;
pub mod barite;
pub mod chalcocite;
pub mod cinnabar;
pub mod fluorite;
pub mod galena;
pub mod gypsum;
pub mod molybdenite;
pub mod monazite;
pub mod orpiment;
pub mod pyrite;
pub mod realgar;
pub mod sphalerite;
pub mod vivianite;

pub fn all() -> Vec<Molecule> {
    vec![
        pyrite::molecule(),
        galena::molecule(),
        sphalerite::molecule(),
        chalcocite::molecule(),
        molybdenite::molecule(),
        cinnabar::molecule(),
        realgar::molecule(),
        orpiment::molecule(),
        gypsum::molecule(),
        barite::molecule(),
        fluorite::molecule(),
        apatite::molecule(),
        monazite::molecule(),
        vivianite::molecule(),
    ]
}
