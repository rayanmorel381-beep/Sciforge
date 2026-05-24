use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H13NO3",
        name: "adrenaline",
        composition: &[(6, 9), (1, 13), (7, 1), (8, 3)],
        molar_mass: 183.207,
        category: "hormone",
        state_at_stp: "solid",
        melting_point_k: Some(484.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.283),
    }
}
