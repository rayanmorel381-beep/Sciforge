use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "K2Cr2O7",
        name: "potassium dichromate",
        composition: &[(8, 7), (19, 2), (24, 2)],
        molar_mass: 294.181,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(671.0),
        boiling_point_k: Some(773.0),
        density_g_cm3: Some(2.676),
    }
}
