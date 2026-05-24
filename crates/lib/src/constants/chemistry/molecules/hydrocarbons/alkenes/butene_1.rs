use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H8",
        name: "1-butene",
        composition: &[(6, 4), (1, 8)],
        molar_mass: 56.107,
        category: "alkene",
        state_at_stp: "gas",
        melting_point_k: Some(87.8),
        boiling_point_k: Some(266.9),
        density_g_cm3: Some(0.595),
    }
}
