use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Mg(OH)2",
        name: "magnesium hydroxide",
        composition: &[(1, 2), (8, 2), (12, 1)],
        molar_mass: 58.320,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(623.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.344),
    }
}
