use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H28O5",
        name: "prednisolone",
        composition: &[(1, 28), (6, 21), (8, 5)],
        molar_mass: 360.444,
        category: "corticosteroid",
        state_at_stp: "solid",
        melting_point_k: Some(508.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
