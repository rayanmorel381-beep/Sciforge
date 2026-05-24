use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HNO3",
        name: "nitric acid",
        composition: &[(1, 1), (7, 1), (8, 3)],
        molar_mass: 63.012,
        category: "inorganic",
        state_at_stp: "liquid",
        melting_point_k: Some(231.0),
        boiling_point_k: Some(356.0),
        density_g_cm3: Some(1.5129),
    }
}
