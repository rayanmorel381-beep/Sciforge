use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H7NO",
        name: "ethanolamine",
        composition: &[(6, 2), (1, 7), (7, 1), (8, 1)],
        molar_mass: 61.084,
        category: "amine",
        state_at_stp: "liquid",
        melting_point_k: Some(283.7),
        boiling_point_k: Some(443.0),
        density_g_cm3: Some(1.012),
    }
}
