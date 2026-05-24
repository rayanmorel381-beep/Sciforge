use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H9NO3",
        name: "threonine",
        composition: &[(6, 4), (1, 9), (7, 1), (8, 3)],
        molar_mass: 119.120,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(529.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.31),
    }
}
