use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H31N3O5",
        name: "lisinopril",
        composition: &[(1, 31), (6, 21), (7, 3), (8, 5)],
        molar_mass: 405.488,
        category: "ace_inhibitor",
        state_at_stp: "solid",
        melting_point_k: Some(421.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.250),
    }
}
