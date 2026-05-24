use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C24H40O5",
        name: "cholic acid",
        composition: &[(6, 24), (1, 40), (8, 5)],
        molar_mass: 408.572,
        category: "bile acid",
        state_at_stp: "solid",
        melting_point_k: Some(471.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.227),
    }
}
