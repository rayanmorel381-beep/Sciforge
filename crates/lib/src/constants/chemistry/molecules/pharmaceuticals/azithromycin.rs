use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C38H72N2O12",
        name: "azithromycin",
        composition: &[(1, 72), (6, 38), (7, 2), (8, 12)],
        molar_mass: 748.984,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(386.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.180),
    }
}
