use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C30H50",
        name: "squalene",
        composition: &[(6, 30), (1, 50)],
        molar_mass: 410.718,
        category: "triterpene",
        state_at_stp: "liquid",
        melting_point_k: Some(198.0),
        boiling_point_k: Some(558.0),
        density_g_cm3: Some(0.858),
    }
}
