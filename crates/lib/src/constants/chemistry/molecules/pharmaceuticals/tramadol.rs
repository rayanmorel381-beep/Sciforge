use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H25NO2",
        name: "tramadol",
        composition: &[(1, 25), (6, 16), (7, 1), (8, 2)],
        molar_mass: 263.376,
        category: "opioid",
        state_at_stp: "solid",
        melting_point_k: Some(454.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.060),
    }
}
