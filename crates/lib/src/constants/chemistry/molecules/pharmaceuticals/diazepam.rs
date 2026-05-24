use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H13ClN2O",
        name: "diazepam",
        composition: &[(1, 13), (6, 16), (7, 2), (8, 1), (17, 1)],
        molar_mass: 284.740,
        category: "benzodiazepine",
        state_at_stp: "solid",
        melting_point_k: Some(404.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.275),
    }
}
