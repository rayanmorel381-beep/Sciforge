use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H13N3O3",
        name: "mebendazole",
        composition: &[(1, 13), (6, 16), (7, 3), (8, 3)],
        molar_mass: 295.293,
        category: "antiparasitic",
        state_at_stp: "solid",
        melting_point_k: Some(561.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.420),
    }
}
