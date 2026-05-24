use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H18",
        name: "decalin",
        composition: &[(6, 10), (1, 18)],
        molar_mass: 138.254,
        category: "cycloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(230.2),
        boiling_point_k: Some(460.4),
        density_g_cm3: Some(0.896),
    }
}
