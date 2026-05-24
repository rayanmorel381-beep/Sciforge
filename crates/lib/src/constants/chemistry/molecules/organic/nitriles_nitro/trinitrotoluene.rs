use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H5N3O6",
        name: "2,4,6-trinitrotoluene",
        composition: &[(6, 7), (1, 5), (7, 3), (8, 6)],
        molar_mass: 227.131,
        category: "nitro",
        state_at_stp: "solid",
        melting_point_k: Some(354.0),
        boiling_point_k: Some(513.0),
        density_g_cm3: Some(1.654),
    }
}
