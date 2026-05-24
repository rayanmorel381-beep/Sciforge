use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3COOH",
        name: "acetic acid",
        composition: &[(6, 2), (1, 4), (8, 2)],
        molar_mass: 60.052,
        category: "carboxylic acid",
        state_at_stp: "liquid",
        melting_point_k: Some(289.8),
        boiling_point_k: Some(391.0),
        density_g_cm3: Some(1.049),
    }
}
