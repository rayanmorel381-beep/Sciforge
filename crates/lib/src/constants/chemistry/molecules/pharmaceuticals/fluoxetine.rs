use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H18F3NO",
        name: "fluoxetine",
        composition: &[(1, 18), (6, 17), (7, 1), (8, 1), (9, 3)],
        molar_mass: 309.327,
        category: "ssri",
        state_at_stp: "solid",
        melting_point_k: Some(431.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.180),
    }
}
