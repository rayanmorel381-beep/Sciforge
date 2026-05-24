use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H5N5O",
        name: "guanine",
        composition: &[(6, 5), (1, 5), (7, 5), (8, 1)],
        molar_mass: 151.127,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(633.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.20),
    }
}
