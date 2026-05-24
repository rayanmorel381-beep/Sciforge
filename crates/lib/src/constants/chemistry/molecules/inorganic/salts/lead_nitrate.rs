use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Pb(NO3)2",
        name: "lead(II) nitrate",
        composition: &[(7, 2), (8, 6), (82, 1)],
        molar_mass: 331.210,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(743.0),
        boiling_point_k: None,
        density_g_cm3: Some(4.530),
    }
}
