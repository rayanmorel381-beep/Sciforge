use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H5Cl",
        name: "chlorobenzene",
        composition: &[(6, 6), (1, 5), (17, 1)],
        molar_mass: 112.557,
        category: "haloarene",
        state_at_stp: "liquid",
        melting_point_k: Some(227.6),
        boiling_point_k: Some(404.9),
        density_g_cm3: Some(1.107),
    }
}
