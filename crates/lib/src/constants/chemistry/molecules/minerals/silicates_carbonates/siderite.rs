use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "FeCO3",
        name: "siderite",
        composition: &[(6, 1), (8, 3), (26, 1)],
        molar_mass: 115.860,
        category: "carbonate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(3.960),
    }
}
