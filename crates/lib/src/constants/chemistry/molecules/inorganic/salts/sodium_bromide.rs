use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaBr",
        name: "sodium bromide",
        composition: &[(11, 1), (35, 1)],
        molar_mass: 102.894,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1020.0),
        boiling_point_k: Some(1663.0),
        density_g_cm3: Some(3.210),
    }
}
