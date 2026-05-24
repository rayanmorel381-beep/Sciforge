use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H8",
        name: "1-pentyne",
        composition: &[(6, 5), (1, 8)],
        molar_mass: 68.118,
        category: "alkyne",
        state_at_stp: "liquid",
        melting_point_k: Some(167.4),
        boiling_point_k: Some(313.3),
        density_g_cm3: Some(0.690),
    }
}
