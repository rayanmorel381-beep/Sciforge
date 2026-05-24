use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3Br",
        name: "bromomethane",
        composition: &[(6, 1), (1, 3), (35, 1)],
        molar_mass: 94.939,
        category: "haloalkane",
        state_at_stp: "gas",
        melting_point_k: Some(179.4),
        boiling_point_k: Some(276.7),
        density_g_cm3: Some(1.6755),
    }
}
