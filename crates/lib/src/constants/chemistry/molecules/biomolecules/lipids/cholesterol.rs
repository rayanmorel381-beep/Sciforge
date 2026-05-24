use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C27H46O",
        name: "cholesterol",
        composition: &[(6, 27), (1, 46), (8, 1)],
        molar_mass: 386.654,
        category: "sterol",
        state_at_stp: "solid",
        melting_point_k: Some(421.4),
        boiling_point_k: Some(633.0),
        density_g_cm3: Some(1.052),
    }
}
