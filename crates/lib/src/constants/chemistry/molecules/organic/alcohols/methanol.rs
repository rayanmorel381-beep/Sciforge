use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3OH",
        name: "methanol",
        composition: &[(6, 1), (1, 4), (8, 1)],
        molar_mass: 32.042,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(175.6),
        boiling_point_k: Some(337.8),
        density_g_cm3: Some(0.7918),
    }
}
