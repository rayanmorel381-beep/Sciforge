use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H21NO2",
        name: "propranolol",
        composition: &[(1, 21), (6, 16), (7, 1), (8, 2)],
        molar_mass: 259.343,
        category: "beta_blocker",
        state_at_stp: "solid",
        melting_point_k: Some(369.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.080),
    }
}
