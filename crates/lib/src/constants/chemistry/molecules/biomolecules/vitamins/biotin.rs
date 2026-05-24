use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H16N2O3S",
        name: "biotin",
        composition: &[(6, 10), (1, 16), (7, 2), (8, 3), (16, 1)],
        molar_mass: 244.310,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(505.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.410),
    }
}
