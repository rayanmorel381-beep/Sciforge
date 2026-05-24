use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C57H104O6",
        name: "triolein",
        composition: &[(6, 57), (1, 104), (8, 6)],
        molar_mass: 885.432,
        category: "triglyceride",
        state_at_stp: "liquid",
        melting_point_k: Some(278.0),
        boiling_point_k: Some(827.0),
        density_g_cm3: Some(0.915),
    }
}
