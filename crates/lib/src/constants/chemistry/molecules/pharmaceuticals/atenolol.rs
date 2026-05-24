use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C14H22N2O3",
        name: "atenolol",
        composition: &[(1, 22), (6, 14), (7, 2), (8, 3)],
        molar_mass: 266.336,
        category: "beta_blocker",
        state_at_stp: "solid",
        melting_point_k: Some(428.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.190),
    }
}
