use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H5N5",
        name: "adenine",
        composition: &[(6, 5), (1, 5), (7, 5)],
        molar_mass: 135.128,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(633.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.6),
    }
}
