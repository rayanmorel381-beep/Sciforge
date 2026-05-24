use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "N2",
        name: "dinitrogen",
        composition: &[(7, 2)],
        molar_mass: 28.014,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(63.15),
        boiling_point_k: Some(77.36),
        density_g_cm3: Some(0.001_251),
    }
}
