use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C20H28N2O5",
        name: "enalapril",
        composition: &[(1, 28), (6, 20), (7, 2), (8, 5)],
        molar_mass: 376.450,
        category: "ace_inhibitor",
        state_at_stp: "solid",
        melting_point_k: Some(417.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.200),
    }
}
