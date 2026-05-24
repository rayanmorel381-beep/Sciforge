use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H18N2O4S",
        name: "penicillin G",
        composition: &[(1, 18), (6, 16), (7, 2), (8, 4), (16, 1)],
        molar_mass: 334.390,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(488.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.410),
    }
}
