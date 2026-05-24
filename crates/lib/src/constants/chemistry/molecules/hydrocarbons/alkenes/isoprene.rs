use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H8",
        name: "isoprene",
        composition: &[(6, 5), (1, 8)],
        molar_mass: 68.118,
        category: "diene",
        state_at_stp: "liquid",
        melting_point_k: Some(127.0),
        boiling_point_k: Some(307.21),
        density_g_cm3: Some(0.681),
    }
}
