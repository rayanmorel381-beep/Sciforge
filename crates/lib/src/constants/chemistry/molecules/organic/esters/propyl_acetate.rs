use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3COOC3H7",
        name: "propyl acetate",
        composition: &[(6, 5), (1, 10), (8, 2)],
        molar_mass: 102.133,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(178.0),
        boiling_point_k: Some(374.6),
        density_g_cm3: Some(0.888),
    }
}
