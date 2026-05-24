use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaCO3",
        name: "calcium carbonate",
        composition: &[(20, 1), (6, 1), (8, 3)],
        molar_mass: 100.086,
        category: "inorganic",
        state_at_stp: "solid",
        melting_point_k: Some(1612.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.711),
    }
}
