use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5NO2",
        name: "niacin",
        composition: &[(6, 6), (1, 5), (7, 1), (8, 2)],
        molar_mass: 123.111,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(509.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.473),
    }
}
