use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Ba(OH)2",
        name: "barium hydroxide",
        composition: &[(1, 2), (8, 2), (56, 1)],
        molar_mass: 171.342,
        category: "base",
        state_at_stp: "solid",
        melting_point_k: Some(573.0),
        boiling_point_k: Some(1053.0),
        density_g_cm3: Some(3.743),
    }
}
