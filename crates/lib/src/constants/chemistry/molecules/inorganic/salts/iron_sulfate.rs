use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "FeSO4",
        name: "iron(II) sulfate",
        composition: &[(8, 4), (16, 1), (26, 1)],
        molar_mass: 151.908,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(953.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.840),
    }
}
