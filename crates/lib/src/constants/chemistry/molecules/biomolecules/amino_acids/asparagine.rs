use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H8N2O3",
        name: "asparagine",
        composition: &[(6, 4), (1, 8), (7, 2), (8, 3)],
        molar_mass: 132.118,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(508.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.543),
    }
}
