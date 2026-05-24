use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H6O3",
        name: "lactic acid",
        composition: &[(6, 3), (1, 6), (8, 3)],
        molar_mass: 90.078,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(291.0),
        boiling_point_k: Some(395.0),
        density_g_cm3: Some(1.206),
    }
}
