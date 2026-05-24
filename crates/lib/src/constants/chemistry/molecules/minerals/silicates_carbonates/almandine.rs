use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Fe3Al2(SiO4)3",
        name: "almandine",
        composition: &[(8, 12), (13, 2), (14, 3), (26, 3)],
        molar_mass: 497.751,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1588.0),
        boiling_point_k: None,
        density_g_cm3: Some(4.320),
    }
}
