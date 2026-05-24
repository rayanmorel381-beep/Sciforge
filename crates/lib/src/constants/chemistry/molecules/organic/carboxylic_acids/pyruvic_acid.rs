use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H4O3",
        name: "pyruvic acid",
        composition: &[(6, 3), (1, 4), (8, 3)],
        molar_mass: 88.062,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(284.6),
        boiling_point_k: Some(438.0),
        density_g_cm3: Some(1.250),
    }
}
