use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H11N",
        name: "diethylamine",
        composition: &[(6, 4), (1, 11), (7, 1)],
        molar_mass: 73.139,
        category: "amine",
        state_at_stp: "liquid",
        melting_point_k: Some(223.0),
        boiling_point_k: Some(328.7),
        density_g_cm3: Some(0.7074),
    }
}
