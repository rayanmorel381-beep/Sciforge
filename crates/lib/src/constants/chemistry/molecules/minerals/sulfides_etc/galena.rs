use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "PbS",
        name: "galena",
        composition: &[(16, 1), (82, 1)],
        molar_mass: 239.266,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: Some(1387.0),
        boiling_point_k: None,
        density_g_cm3: Some(7.600),
    }
}
