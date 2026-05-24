use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MgCO3",
        name: "magnesium carbonate",
        composition: &[(6, 1), (8, 3), (12, 1)],
        molar_mass: 84.314,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(663.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.958),
    }
}
