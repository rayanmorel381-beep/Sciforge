use super::super::Molecule;

pub mod acetylcholine;
pub mod adrenaline;
pub mod dopamine;
pub mod gaba;
pub mod histamine;
pub mod melatonin;
pub mod noradrenaline;
pub mod oxytocin;
pub mod serotonin;
pub mod thyroxine;

pub fn all() -> Vec<Molecule> {
    vec![
        dopamine::molecule(),
        serotonin::molecule(),
        adrenaline::molecule(),
        noradrenaline::molecule(),
        acetylcholine::molecule(),
        gaba::molecule(),
        histamine::molecule(),
        melatonin::molecule(),
        thyroxine::molecule(),
        oxytocin::molecule(),
    ]
}
