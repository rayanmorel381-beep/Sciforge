use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H11NO3",
        name: "pyridoxine",
        composition: &[(6, 8), (1, 11), (7, 1), (8, 3)],
        molar_mass: 169.180,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(433.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.270),
    }
}
