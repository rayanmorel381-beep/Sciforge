use super::super::Molecule;

pub mod calcium_oxide;
pub mod chlorine_dioxide;
pub mod chromium_oxide;
pub mod copper_oxide;
pub mod dichlorine_monoxide;
pub mod dinitrogen_pentoxide;
pub mod iron_oxide;
pub mod magnetite;
pub mod nitric_oxide;
pub mod nitrogen_dioxide;
pub mod nitrous_oxide;
pub mod phosphorus_pentoxide;
pub mod sulfur_dioxide;
pub mod sulfur_trioxide;
pub mod zinc_oxide;
pub mod water;
pub mod carbon_dioxide;
pub mod carbon_monoxide;

pub fn all() -> Vec<Molecule> {
    vec![
        water::molecule(),
        carbon_dioxide::molecule(),
        carbon_monoxide::molecule(),
        sulfur_dioxide::molecule(),
        sulfur_trioxide::molecule(),
        nitric_oxide::molecule(),
        nitrogen_dioxide::molecule(),
        nitrous_oxide::molecule(),
        dinitrogen_pentoxide::molecule(),
        phosphorus_pentoxide::molecule(),
        dichlorine_monoxide::molecule(),
        chlorine_dioxide::molecule(),
        calcium_oxide::molecule(),
        zinc_oxide::molecule(),
        copper_oxide::molecule(),
        iron_oxide::molecule(),
        magnetite::molecule(),
        chromium_oxide::molecule(),
    ]
}
