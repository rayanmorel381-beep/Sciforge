use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H11N3O3S",
        name: "sulfamethoxazole",
        composition: &[(1, 11), (6, 10), (7, 3), (8, 3), (16, 1)],
        molar_mass: 253.279,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(440.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.490),
    }
}
