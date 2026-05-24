use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H8",
        name: "cyclobutane",
        composition: &[(6, 4), (1, 8)],
        molar_mass: 56.107,
        category: "cycloalkane",
        state_at_stp: "gas",
        melting_point_k: Some(182.0),
        boiling_point_k: Some(285.7),
        density_g_cm3: Some(0.720),
    }
}
