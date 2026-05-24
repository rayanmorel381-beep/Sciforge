use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H13NO2",
        name: "leucine",
        composition: &[(6, 6), (1, 13), (7, 1), (8, 2)],
        molar_mass: 131.174,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(566.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.293),
    }
}
