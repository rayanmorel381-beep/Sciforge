use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3SH",
        name: "methanethiol",
        composition: &[(6, 1), (1, 4), (16, 1)],
        molar_mass: 48.108,
        category: "thiol",
        state_at_stp: "gas",
        melting_point_k: Some(150.2),
        boiling_point_k: Some(279.1),
        density_g_cm3: Some(0.866),
    }
}
