use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Al(OH)3",
        name: "aluminum hydroxide",
        composition: &[(1, 3), (8, 3), (13, 1)],
        molar_mass: 78.004,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(573.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.420),
    }
}
