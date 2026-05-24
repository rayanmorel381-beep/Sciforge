use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H8O",
        name: "anisole",
        composition: &[(6, 7), (1, 8), (8, 1)],
        molar_mass: 108.140,
        category: "ether",
        state_at_stp: "liquid",
        melting_point_k: Some(235.6),
        boiling_point_k: Some(427.0),
        density_g_cm3: Some(0.995),
    }
}
