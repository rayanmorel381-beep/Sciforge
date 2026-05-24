use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H6",
        name: "cyclopropane",
        composition: &[(6, 3), (1, 6)],
        molar_mass: 42.080,
        category: "cycloalkane",
        state_at_stp: "gas",
        melting_point_k: Some(145.7),
        boiling_point_k: Some(240.4),
        density_g_cm3: Some(0.001879),
    }
}
