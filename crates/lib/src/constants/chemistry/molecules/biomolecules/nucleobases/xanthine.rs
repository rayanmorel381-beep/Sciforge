use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H4N4O2",
        name: "xanthine",
        composition: &[(6, 5), (1, 4), (7, 4), (8, 2)],
        molar_mass: 152.111,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(486.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.6),
    }
}
