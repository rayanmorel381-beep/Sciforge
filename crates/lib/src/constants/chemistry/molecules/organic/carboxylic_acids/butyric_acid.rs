use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H7COOH",
        name: "butyric acid",
        composition: &[(6, 4), (1, 8), (8, 2)],
        molar_mass: 88.106,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(267.9),
        boiling_point_k: Some(436.4),
        density_g_cm3: Some(0.960),
    }
}
