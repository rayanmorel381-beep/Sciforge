use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H10O",
        name: "diethyl ether",
        composition: &[(6, 4), (1, 10), (8, 1)],
        molar_mass: 74.123,
        category: "ether",
        state_at_stp: "liquid",
        melting_point_k: Some(156.9),
        boiling_point_k: Some(307.6),
        density_g_cm3: Some(0.713),
    }
}
