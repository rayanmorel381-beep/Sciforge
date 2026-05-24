use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Na2CO3",
        name: "sodium carbonate",
        composition: &[(6, 1), (8, 3), (11, 2)],
        molar_mass: 105.988,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1124.0),
        boiling_point_k: Some(1873.0),
        density_g_cm3: Some(2.540),
    }
}
