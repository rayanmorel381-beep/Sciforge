use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CO2",
        name: "carbon dioxide",
        composition: &[(6, 1), (8, 2)],
        molar_mass: 44.009,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(216.6),
        boiling_point_k: Some(194.7),
        density_g_cm3: Some(0.001_977),
    }
}
