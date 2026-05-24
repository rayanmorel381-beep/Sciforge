use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaMgSi2O6",
        name: "diopside",
        composition: &[(8, 6), (12, 1), (14, 2), (20, 1)],
        molar_mass: 216.550,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1664.0),
        boiling_point_k: None,
        density_g_cm3: Some(3.400),
    }
}
