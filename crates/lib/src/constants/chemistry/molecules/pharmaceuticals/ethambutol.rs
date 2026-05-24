use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H24N2O2",
        name: "ethambutol",
        composition: &[(1, 24), (6, 10), (7, 2), (8, 2)],
        molar_mass: 204.310,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(361.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.020),
    }
}
