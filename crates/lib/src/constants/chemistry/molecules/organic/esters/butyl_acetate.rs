use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3COOC4H9",
        name: "butyl acetate",
        composition: &[(6, 6), (1, 12), (8, 2)],
        molar_mass: 116.160,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(196.0),
        boiling_point_k: Some(399.0),
        density_g_cm3: Some(0.882),
    }
}
