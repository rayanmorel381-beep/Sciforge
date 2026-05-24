use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HNO2",
        name: "nitrous acid",
        composition: &[(1, 1), (7, 1), (8, 2)],
        molar_mass: 47.013,
        category: "acid",
        state_at_stp: "liquid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(1.000),
    }
}
