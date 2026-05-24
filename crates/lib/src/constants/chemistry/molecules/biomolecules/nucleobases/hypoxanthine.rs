use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H4N4O",
        name: "hypoxanthine",
        composition: &[(6, 5), (1, 4), (7, 4), (8, 1)],
        molar_mass: 136.112,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(423.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.0),
    }
}
