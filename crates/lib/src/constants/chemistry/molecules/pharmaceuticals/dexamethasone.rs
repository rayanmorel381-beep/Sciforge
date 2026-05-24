use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C22H29FO5",
        name: "dexamethasone",
        composition: &[(1, 29), (6, 22), (8, 5), (9, 1)],
        molar_mass: 392.461,
        category: "corticosteroid",
        state_at_stp: "solid",
        melting_point_k: Some(535.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.330),
    }
}
