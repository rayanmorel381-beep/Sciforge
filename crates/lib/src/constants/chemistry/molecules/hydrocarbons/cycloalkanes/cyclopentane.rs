use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H10",
        name: "cyclopentane",
        composition: &[(6, 5), (1, 10)],
        molar_mass: 70.133,
        category: "cycloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(179.3),
        boiling_point_k: Some(322.4),
        density_g_cm3: Some(0.751),
    }
}
