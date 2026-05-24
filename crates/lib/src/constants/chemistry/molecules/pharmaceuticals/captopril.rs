use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H15NO3S",
        name: "captopril",
        composition: &[(1, 15), (6, 9), (7, 1), (8, 3), (16, 1)],
        molar_mass: 217.290,
        category: "ace_inhibitor",
        state_at_stp: "solid",
        melting_point_k: Some(379.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.230),
    }
}
