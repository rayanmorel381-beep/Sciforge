use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H3NS",
        name: "thiazole",
        composition: &[(6, 3), (1, 3), (7, 1), (16, 1)],
        molar_mass: 85.128,
        category: "heterocycle",
        state_at_stp: "liquid",
        melting_point_k: Some(240.0),
        boiling_point_k: Some(391.5),
        density_g_cm3: Some(1.200),
    }
}
