use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "ZrSiO4",
        name: "zircon",
        composition: &[(8, 4), (14, 1), (40, 1)],
        molar_mass: 183.310,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(2823.0),
        boiling_point_k: None,
        density_g_cm3: Some(4.650),
    }
}
