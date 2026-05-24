use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H23ClFNO2",
        name: "haloperidol",
        composition: &[(1, 23), (6, 21), (7, 1), (8, 2), (9, 1), (17, 1)],
        molar_mass: 375.864,
        category: "antipsychotic",
        state_at_stp: "solid",
        melting_point_k: Some(425.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
