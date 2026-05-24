use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "KOH",
        name: "potassium hydroxide",
        composition: &[(1, 1), (8, 1), (19, 1)],
        molar_mass: 56.106,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(633.0),
        boiling_point_k: Some(1600.0),
        density_g_cm3: Some(2.044),
    }
}
