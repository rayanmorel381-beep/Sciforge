use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2SO3",
        name: "sulfurous acid",
        composition: &[(1, 2), (8, 3), (16, 1)],
        molar_mass: 82.073,
        category: "acid",
        state_at_stp: "liquid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.030),
    }
}
