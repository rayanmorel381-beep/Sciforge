use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H9NO3",
        name: "hydroxyproline",
        composition: &[(6, 5), (1, 9), (7, 1), (8, 3)],
        molar_mass: 131.130,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: Some(543.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
