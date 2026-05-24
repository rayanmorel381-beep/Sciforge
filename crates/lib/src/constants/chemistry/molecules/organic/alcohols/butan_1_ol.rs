use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H9OH",
        name: "butan-1-ol",
        composition: &[(6, 4), (1, 10), (8, 1)],
        molar_mass: 74.123,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(183.6),
        boiling_point_k: Some(390.8),
        density_g_cm3: Some(0.810),
    }
}
