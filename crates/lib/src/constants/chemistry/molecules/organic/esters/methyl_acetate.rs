use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3COOCH3",
        name: "methyl acetate",
        composition: &[(6, 3), (1, 6), (8, 2)],
        molar_mass: 74.079,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(175.0),
        boiling_point_k: Some(330.0),
        density_g_cm3: Some(0.932),
    }
}
