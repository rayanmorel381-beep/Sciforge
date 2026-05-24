use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H9COOH",
        name: "valeric acid",
        composition: &[(6, 5), (1, 10), (8, 2)],
        molar_mass: 102.133,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(239.2),
        boiling_point_k: Some(459.0),
        density_g_cm3: Some(0.930),
    }
}
