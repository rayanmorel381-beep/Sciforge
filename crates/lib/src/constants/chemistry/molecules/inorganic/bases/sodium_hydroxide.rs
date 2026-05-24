use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaOH",
        name: "sodium hydroxide",
        composition: &[(1, 1), (8, 1), (11, 1)],
        molar_mass: 39.997,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(591.0),
        boiling_point_k: Some(1661.0),
        density_g_cm3: Some(2.130),
    }
}
