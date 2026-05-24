use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5COOH",
        name: "benzoic acid",
        composition: &[(6, 7), (1, 6), (8, 2)],
        molar_mass: 122.123,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(395.5),
        boiling_point_k: Some(522.4),
        density_g_cm3: Some(1.270),
    }
}
