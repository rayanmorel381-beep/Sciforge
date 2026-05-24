use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H17NO5",
        name: "pantothenic acid",
        composition: &[(6, 9), (1, 17), (7, 1), (8, 5)],
        molar_mass: 219.235,
        category: "vitamin",
        state_at_stp: "liquid",
        melting_point_k: Some(346.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.266),
    }
}
