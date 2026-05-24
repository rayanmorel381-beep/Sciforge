use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H5COOH",
        name: "propionic acid",
        composition: &[(6, 3), (1, 6), (8, 2)],
        molar_mass: 74.079,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(252.2),
        boiling_point_k: Some(414.3),
        density_g_cm3: Some(0.992),
    }
}
