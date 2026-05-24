use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2CO3",
        name: "carbonic acid",
        composition: &[(1, 2), (6, 1), (8, 3)],
        molar_mass: 62.025,
        category: "acid",
        state_at_stp: "liquid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.668),
    }
}
