use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C28H44O",
        name: "ergosterol",
        composition: &[(6, 28), (1, 44), (8, 1)],
        molar_mass: 396.659,
        category: "sterol",
        state_at_stp: "solid",
        melting_point_k: Some(433.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.000),
    }
}
