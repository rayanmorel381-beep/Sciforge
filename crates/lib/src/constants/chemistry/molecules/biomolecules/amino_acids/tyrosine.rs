use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H11NO3",
        name: "tyrosine",
        composition: &[(6, 9), (1, 11), (7, 1), (8, 3)],
        molar_mass: 181.191,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(616.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.456),
    }
}
