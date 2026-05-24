use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H4N2O2",
        name: "uracil",
        composition: &[(6, 4), (1, 4), (7, 2), (8, 2)],
        molar_mass: 112.087,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(608.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.32),
    }
}
