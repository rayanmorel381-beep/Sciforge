use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H16",
        name: "cyclooctane",
        composition: &[(6, 8), (1, 16)],
        molar_mass: 112.213,
        category: "cycloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(287.1),
        boiling_point_k: Some(422.0),
        density_g_cm3: Some(0.834),
    }
}
