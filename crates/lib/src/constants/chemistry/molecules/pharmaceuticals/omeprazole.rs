use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H19N3O3S",
        name: "omeprazole",
        composition: &[(1, 19), (6, 17), (7, 3), (8, 3), (16, 1)],
        molar_mass: 345.420,
        category: "ppi",
        state_at_stp: "solid",
        melting_point_k: Some(429.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.400),
    }
}
