use super::super::Molecule;

pub mod dimethyl_disulfide;
pub mod dimethyl_sulfide;
pub mod dimethyl_sulfoxide;
pub mod ethanethiol;
pub mod methanethiol;
pub mod thiophene;

pub fn all() -> Vec<Molecule> {
    vec![
        methanethiol::molecule(),
        ethanethiol::molecule(),
        dimethyl_sulfide::molecule(),
        dimethyl_disulfide::molecule(),
        dimethyl_sulfoxide::molecule(),
        thiophene::molecule(),
    ]
}
