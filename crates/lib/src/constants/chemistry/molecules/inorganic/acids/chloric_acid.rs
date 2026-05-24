use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HClO3",
        name: "chloric acid",
        composition: &[(1, 1), (8, 3), (17, 1)],
        molar_mass: 84.459,
        category: "acid",
        state_at_stp: "liquid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.282),
    }
}
