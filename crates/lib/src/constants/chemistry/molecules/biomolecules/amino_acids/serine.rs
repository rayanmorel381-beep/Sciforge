use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7NO3",
        name: "serine",
        composition: &[(6, 3), (1, 7), (7, 1), (8, 3)],
        molar_mass: 105.093,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(519.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.537),
    }
}
