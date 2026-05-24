use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MgSO4",
        name: "magnesium sulfate",
        composition: &[(8, 4), (12, 1), (16, 1)],
        molar_mass: 120.366,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1397.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.660),
    }
}
