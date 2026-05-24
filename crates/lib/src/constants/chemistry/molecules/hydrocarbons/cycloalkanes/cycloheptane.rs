use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H14",
        name: "cycloheptane",
        composition: &[(6, 7), (1, 14)],
        molar_mass: 98.186,
        category: "cycloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(261.0),
        boiling_point_k: Some(391.9),
        density_g_cm3: Some(0.811),
    }
}
