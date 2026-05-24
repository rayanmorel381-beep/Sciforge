use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7NO2S",
        name: "cysteine",
        composition: &[(6, 3), (1, 7), (7, 1), (8, 2), (16, 1)],
        molar_mass: 121.158,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(513.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.677),
    }
}
