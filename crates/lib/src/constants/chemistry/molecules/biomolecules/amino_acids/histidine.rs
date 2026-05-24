use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H9N3O2",
        name: "histidine",
        composition: &[(6, 6), (1, 9), (7, 3), (8, 2)],
        molar_mass: 155.155,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(560.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.42),
    }
}
