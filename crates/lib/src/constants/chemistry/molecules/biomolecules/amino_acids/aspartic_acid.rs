use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H7NO4",
        name: "aspartic acid",
        composition: &[(6, 4), (1, 7), (7, 1), (8, 4)],
        molar_mass: 133.103,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(543.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.7),
    }
}
