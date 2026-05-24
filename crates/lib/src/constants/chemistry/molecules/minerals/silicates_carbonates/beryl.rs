use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Be3Al2Si6O18",
        name: "beryl",
        composition: &[(4, 3), (8, 18), (13, 2), (14, 6)],
        molar_mass: 537.500,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1923.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.760),
    }
}
