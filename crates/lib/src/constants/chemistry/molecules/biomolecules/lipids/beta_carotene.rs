use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C40H56",
        name: "beta-carotene",
        composition: &[(6, 40), (1, 56)],
        molar_mass: 536.873,
        category: "carotenoid",
        state_at_stp: "solid",
        melting_point_k: Some(456.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.000),
    }
}
