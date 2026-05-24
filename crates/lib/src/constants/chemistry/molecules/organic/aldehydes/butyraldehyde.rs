use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7CHO",
        name: "butyraldehyde",
        composition: &[(6, 4), (1, 8), (8, 1)],
        molar_mass: 72.107,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(176.8),
        boiling_point_k: Some(347.8),
        density_g_cm3: Some(0.802),
    }
}
