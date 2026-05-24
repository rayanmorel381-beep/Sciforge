use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H5N3O",
        name: "cytosine",
        composition: &[(6, 4), (1, 5), (7, 3), (8, 1)],
        molar_mass: 111.102,
        category: "nucleobase",
        state_at_stp: "solid",
        melting_point_k: Some(593.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.55),
    }
}
