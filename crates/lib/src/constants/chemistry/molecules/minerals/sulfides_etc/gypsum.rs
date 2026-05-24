use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaSO4·2H2O",
        name: "gypsum",
        composition: &[(1, 4), (8, 6), (16, 1), (20, 1)],
        molar_mass: 172.172,
        category: "sulfate",
        state_at_stp: "solid",
        melting_point_k: Some(423.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.320),
    }
}
