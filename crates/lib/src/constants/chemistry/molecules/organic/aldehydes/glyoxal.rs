use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H2O2",
        name: "glyoxal",
        composition: &[(6, 2), (1, 2), (8, 2)],
        molar_mass: 58.036,
        category: "aldehyde",
        state_at_stp: "liquid",
        melting_point_k: Some(288.0),
        boiling_point_k: Some(324.0),
        density_g_cm3: Some(1.27),
    }
}
