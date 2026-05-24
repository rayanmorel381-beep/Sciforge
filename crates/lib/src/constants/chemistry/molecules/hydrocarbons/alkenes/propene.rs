use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H6",
        name: "propene",
        composition: &[(6, 3), (1, 6)],
        molar_mass: 42.080,
        category: "alkene",
        state_at_stp: "gas",
        melting_point_k: Some(87.9),
        boiling_point_k: Some(225.46),
        density_g_cm3: Some(0.001745),
    }
}
