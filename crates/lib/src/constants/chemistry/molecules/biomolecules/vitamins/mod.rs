use super::super::Molecule;

pub mod alpha_tocopherol;
pub mod ascorbic_acid;
pub mod biotin;
pub mod cholecalciferol;
pub mod cobalamin;
pub mod folic_acid;
pub mod niacin;
pub mod pantothenic_acid;
pub mod phylloquinone;
pub mod pyridoxine;
pub mod retinol;
pub mod riboflavin;
pub mod thiamine;

pub fn all() -> Vec<Molecule> {
    vec![
        retinol::molecule(),
        thiamine::molecule(),
        riboflavin::molecule(),
        niacin::molecule(),
        pantothenic_acid::molecule(),
        pyridoxine::molecule(),
        biotin::molecule(),
        folic_acid::molecule(),
        cobalamin::molecule(),
        ascorbic_acid::molecule(),
        cholecalciferol::molecule(),
        alpha_tocopherol::molecule(),
        phylloquinone::molecule(),
    ]
}
