use super::super::Molecule;

pub mod acetone;
pub mod acetophenone;
pub mod benzophenone;
pub mod butanone;
pub mod cyclohexanone;
pub mod hexan_2_one;
pub mod pentan_2_one;
pub mod pentan_3_one;

pub fn all() -> Vec<Molecule> {
    vec![
        acetone::molecule(),
        butanone::molecule(),
        pentan_2_one::molecule(),
        pentan_3_one::molecule(),
        hexan_2_one::molecule(),
        cyclohexanone::molecule(),
        acetophenone::molecule(),
        benzophenone::molecule(),
    ]
}
