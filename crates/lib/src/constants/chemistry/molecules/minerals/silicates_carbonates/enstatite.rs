use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MgSiO3",
        name: "enstatite",
        composition: &[(8, 3), (12, 1), (14, 1)],
        molar_mass: 100.389,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1830.0),
        boiling_point_k: None,
        density_g_cm3: Some(3.200),
    }
}
