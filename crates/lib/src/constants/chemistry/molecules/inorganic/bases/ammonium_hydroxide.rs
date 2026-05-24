use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NH4OH",
        name: "ammonium hydroxide",
        composition: &[(1, 5), (7, 1), (8, 1)],
        molar_mass: 35.046,
        category: "base",
        state_at_stp: "liquid",
        melting_point_k: Some(216.0),
        boiling_point_k: Some(311.0),
        density_g_cm3: Some(0.880),
    }
}
