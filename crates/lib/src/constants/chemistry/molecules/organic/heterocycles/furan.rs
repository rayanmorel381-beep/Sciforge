use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H4O",
        name: "furan",
        composition: &[(6, 4), (1, 4), (8, 1)],
        molar_mass: 68.074,
        category: "heterocycle",
        state_at_stp: "liquid",
        melting_point_k: Some(187.5),
        boiling_point_k: Some(304.5),
        density_g_cm3: Some(0.936),
    }
}
