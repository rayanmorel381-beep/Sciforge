use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H21NO3",
        name: "codeine",
        composition: &[(1, 21), (6, 18), (7, 1), (8, 3)],
        molar_mass: 299.370,
        category: "opioid",
        state_at_stp: "solid",
        melting_point_k: Some(429.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.320),
    }
}
