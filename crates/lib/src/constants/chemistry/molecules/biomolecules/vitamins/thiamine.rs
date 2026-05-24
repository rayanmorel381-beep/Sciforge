use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H17N4OS",
        name: "thiamine",
        composition: &[(6, 12), (1, 17), (7, 4), (8, 1), (16, 1)],
        molar_mass: 265.355,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(521.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
