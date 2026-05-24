use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C22H23ClN2O2",
        name: "loratadine",
        composition: &[(1, 23), (6, 22), (7, 2), (8, 2), (17, 1)],
        molar_mass: 382.883,
        category: "antihistamine",
        state_at_stp: "solid",
        melting_point_k: Some(407.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.200),
    }
}
