use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C20H25ClN2O5",
        name: "amlodipine",
        composition: &[(1, 25), (6, 20), (7, 2), (8, 5), (17, 1)],
        molar_mass: 408.879,
        category: "calcium_blocker",
        state_at_stp: "solid",
        melting_point_k: Some(472.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.260),
    }
}
