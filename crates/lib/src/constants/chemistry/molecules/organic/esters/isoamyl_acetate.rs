use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H14O2",
        name: "isoamyl acetate",
        composition: &[(6, 7), (1, 14), (8, 2)],
        molar_mass: 130.187,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(195.0),
        boiling_point_k: Some(415.0),
        density_g_cm3: Some(0.876),
    }
}
