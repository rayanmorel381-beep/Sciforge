use super::super::Molecule;

pub mod adenine;
pub mod cytosine;
pub mod guanine;
pub mod hypoxanthine;
pub mod methylcytosine_5;
pub mod thymine;
pub mod uracil;
pub mod xanthine;

pub fn all() -> Vec<Molecule> {
    vec![
        adenine::molecule(),
        guanine::molecule(),
        cytosine::molecule(),
        thymine::molecule(),
        uracil::molecule(),
        hypoxanthine::molecule(),
        xanthine::molecule(),
        methylcytosine_5::molecule(),
    ]
}
