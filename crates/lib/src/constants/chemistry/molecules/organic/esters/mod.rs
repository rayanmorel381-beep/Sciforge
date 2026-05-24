use super::super::Molecule;

pub mod butyl_acetate;
pub mod ethyl_acetate;
pub mod ethyl_butyrate;
pub mod ethyl_formate;
pub mod isoamyl_acetate;
pub mod methyl_acetate;
pub mod methyl_benzoate;
pub mod methyl_formate;
pub mod methyl_methacrylate;
pub mod methyl_salicylate;
pub mod propyl_acetate;
pub mod vinyl_acetate;

pub fn all() -> Vec<Molecule> {
    vec![
        methyl_formate::molecule(),
        ethyl_formate::molecule(),
        methyl_acetate::molecule(),
        ethyl_acetate::molecule(),
        propyl_acetate::molecule(),
        butyl_acetate::molecule(),
        isoamyl_acetate::molecule(),
        ethyl_butyrate::molecule(),
        methyl_benzoate::molecule(),
        methyl_salicylate::molecule(),
        vinyl_acetate::molecule(),
        methyl_methacrylate::molecule(),
    ]
}
