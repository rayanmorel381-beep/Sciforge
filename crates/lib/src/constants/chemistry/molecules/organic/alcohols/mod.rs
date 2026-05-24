use super::super::Molecule;

pub mod benzyl_alcohol;
pub mod butan_1_ol;
pub mod butan_2_ol;
pub mod ethanol;
pub mod ethylene_glycol;
pub mod glycerol;
pub mod hexan_1_ol;
pub mod isobutanol;
pub mod methanol;
pub mod pentan_1_ol;
pub mod phenol;
pub mod propan_1_ol;
pub mod propan_2_ol;
pub mod propylene_glycol;
pub mod tert_butanol;

pub fn all() -> Vec<Molecule> {
    vec![
        methanol::molecule(),
        ethanol::molecule(),
        propan_1_ol::molecule(),
        propan_2_ol::molecule(),
        butan_1_ol::molecule(),
        butan_2_ol::molecule(),
        isobutanol::molecule(),
        tert_butanol::molecule(),
        pentan_1_ol::molecule(),
        hexan_1_ol::molecule(),
        ethylene_glycol::molecule(),
        propylene_glycol::molecule(),
        glycerol::molecule(),
        phenol::molecule(),
        benzyl_alcohol::molecule(),
    ]
}
