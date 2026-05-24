use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3COOC2H5",
        name: "ethyl acetate",
        composition: &[(6, 4), (1, 8), (8, 2)],
        molar_mass: 88.106,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(189.6),
        boiling_point_k: Some(350.2),
        density_g_cm3: Some(0.902),
    }
}
