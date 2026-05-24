use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H17Cl2N",
        name: "sertraline",
        composition: &[(1, 17), (6, 17), (7, 1), (17, 2)],
        molar_mass: 306.234,
        category: "ssri",
        state_at_stp: "solid",
        melting_point_k: Some(519.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
