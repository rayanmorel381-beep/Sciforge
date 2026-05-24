use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH3I",
        name: "iodomethane",
        composition: &[(6, 1), (1, 3), (53, 1)],
        molar_mass: 141.939,
        category: "haloalkane",
        state_at_stp: "liquid",
        melting_point_k: Some(206.7),
        boiling_point_k: Some(315.6),
        density_g_cm3: Some(2.280),
    }
}
