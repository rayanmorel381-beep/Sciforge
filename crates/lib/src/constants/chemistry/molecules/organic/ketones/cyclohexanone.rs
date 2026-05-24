use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H10O",
        name: "cyclohexanone",
        composition: &[(6, 6), (1, 10), (8, 1)],
        molar_mass: 98.145,
        category: "ketone",
        state_at_stp: "liquid",
        melting_point_k: Some(241.0),
        boiling_point_k: Some(429.0),
        density_g_cm3: Some(0.948),
    }
}
