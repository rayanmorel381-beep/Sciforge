use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C19H28O2",
        name: "testosterone",
        composition: &[(6, 19), (1, 28), (8, 2)],
        molar_mass: 288.424,
        category: "steroid hormone",
        state_at_stp: "solid",
        melting_point_k: Some(428.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.110),
    }
}
