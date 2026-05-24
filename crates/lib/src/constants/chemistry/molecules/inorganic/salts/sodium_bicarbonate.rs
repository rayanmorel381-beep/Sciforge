use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaHCO3",
        name: "sodium bicarbonate",
        composition: &[(1, 1), (6, 1), (8, 3), (11, 1)],
        molar_mass: 84.007,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(323.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.200),
    }
}
