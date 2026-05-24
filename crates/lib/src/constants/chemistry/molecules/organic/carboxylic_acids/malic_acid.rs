use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6O5",
        name: "malic acid",
        composition: &[(6, 4), (1, 6), (8, 5)],
        molar_mass: 134.087,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(403.0),
        boiling_point_k: Some(423.0),
        density_g_cm3: Some(1.609),
    }
}
