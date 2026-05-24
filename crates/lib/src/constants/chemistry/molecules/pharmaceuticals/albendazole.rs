use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H15N3O2S",
        name: "albendazole",
        composition: &[(1, 15), (6, 12), (7, 3), (8, 2), (16, 1)],
        molar_mass: 265.331,
        category: "antiparasitic",
        state_at_stp: "solid",
        melting_point_k: Some(481.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.460),
    }
}
