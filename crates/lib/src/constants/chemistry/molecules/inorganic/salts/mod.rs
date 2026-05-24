use super::super::Molecule;

pub mod ammonium_chloride;
pub mod calcium_chloride;
pub mod calcium_phosphate;
pub mod calcium_sulfate;
pub mod copper_sulfate;
pub mod iron_sulfate;
pub mod lead_nitrate;
pub mod magnesium_carbonate;
pub mod magnesium_chloride;
pub mod magnesium_sulfate;
pub mod potassium_bromide;
pub mod potassium_carbonate;
pub mod potassium_chloride;
pub mod potassium_dichromate;
pub mod potassium_fluoride;
pub mod potassium_iodide;
pub mod potassium_nitrate;
pub mod potassium_permanganate;
pub mod potassium_phosphate;
pub mod potassium_sulfate;
pub mod silver_nitrate;
pub mod sodium_bicarbonate;
pub mod sodium_bromide;
pub mod sodium_carbonate;
pub mod sodium_fluoride;
pub mod sodium_iodide;
pub mod sodium_nitrate;
pub mod sodium_phosphate;
pub mod sodium_sulfate;
pub mod zinc_sulfate;
pub mod sodium_chloride;
pub mod calcium_carbonate;

pub fn all() -> Vec<Molecule> {
    vec![
        sodium_chloride::molecule(),
        calcium_carbonate::molecule(),
        potassium_chloride::molecule(),
        potassium_bromide::molecule(),
        potassium_iodide::molecule(),
        potassium_fluoride::molecule(),
        sodium_bromide::molecule(),
        sodium_iodide::molecule(),
        sodium_fluoride::molecule(),
        calcium_chloride::molecule(),
        magnesium_chloride::molecule(),
        sodium_bicarbonate::molecule(),
        sodium_carbonate::molecule(),
        potassium_carbonate::molecule(),
        magnesium_carbonate::molecule(),
        sodium_nitrate::molecule(),
        potassium_nitrate::molecule(),
        calcium_sulfate::molecule(),
        magnesium_sulfate::molecule(),
        sodium_sulfate::molecule(),
        potassium_sulfate::molecule(),
        copper_sulfate::molecule(),
        iron_sulfate::molecule(),
        zinc_sulfate::molecule(),
        silver_nitrate::molecule(),
        lead_nitrate::molecule(),
        sodium_phosphate::molecule(),
        potassium_phosphate::molecule(),
        calcium_phosphate::molecule(),
        ammonium_chloride::molecule(),
        potassium_permanganate::molecule(),
        potassium_dichromate::molecule(),
    ]
}
