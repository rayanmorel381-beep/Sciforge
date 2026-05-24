use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaH",
        name: "sodium hydride",
        composition: &[(1, 1), (11, 1)],
        molar_mass: 23.998,
        category: "hydride",
        state_at_stp: "solid",
        melting_point_k: Some(911.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.396),
    }
}
