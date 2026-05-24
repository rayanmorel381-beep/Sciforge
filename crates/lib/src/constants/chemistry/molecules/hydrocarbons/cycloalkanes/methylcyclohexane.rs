use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H14",
        name: "methylcyclohexane",
        composition: &[(6, 7), (1, 14)],
        molar_mass: 98.186,
        category: "cycloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(146.4),
        boiling_point_k: Some(374.0),
        density_g_cm3: Some(0.770),
    }
}
