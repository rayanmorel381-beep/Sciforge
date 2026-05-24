use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C33H35FN2O5",
        name: "atorvastatin",
        composition: &[(1, 35), (6, 33), (7, 2), (8, 5), (9, 1)],
        molar_mass: 558.640,
        category: "statin",
        state_at_stp: "solid",
        melting_point_k: Some(432.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.220),
    }
}
