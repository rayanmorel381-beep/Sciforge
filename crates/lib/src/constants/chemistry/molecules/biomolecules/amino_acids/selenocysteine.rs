use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7NO2Se",
        name: "selenocysteine",
        composition: &[(6, 3), (1, 7), (7, 1), (8, 2), (34, 1)],
        molar_mass: 168.053,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.800),
    }
}
