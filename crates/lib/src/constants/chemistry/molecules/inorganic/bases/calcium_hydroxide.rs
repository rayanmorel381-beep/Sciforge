use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Ca(OH)2",
        name: "calcium hydroxide",
        composition: &[(1, 2), (8, 2), (20, 1)],
        molar_mass: 74.093,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(853.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.211),
    }
}
