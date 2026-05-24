use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7NO2",
        name: "sarcosine",
        composition: &[(6, 3), (1, 7), (7, 1), (8, 2)],
        molar_mass: 89.094,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: Some(481.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.097),
    }
}
