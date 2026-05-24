use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaH2",
        name: "calcium hydride",
        composition: &[(1, 2), (20, 1)],
        molar_mass: 42.094,
        category: "hydride",
        state_at_stp: "solid",
        melting_point_k: Some(1089.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.700),
    }
}
