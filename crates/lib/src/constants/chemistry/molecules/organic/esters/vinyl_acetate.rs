use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6O2",
        name: "vinyl acetate",
        composition: &[(6, 4), (1, 6), (8, 2)],
        molar_mass: 86.090,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(180.4),
        boiling_point_k: Some(345.7),
        density_g_cm3: Some(0.934),
    }
}
