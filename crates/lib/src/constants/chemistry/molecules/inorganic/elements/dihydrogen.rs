use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2",
        name: "dihydrogen",
        composition: &[(1, 2)],
        molar_mass: 2.016,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(13.99),
        boiling_point_k: Some(20.27),
        density_g_cm3: Some(0.000_089_88),
    }
}
