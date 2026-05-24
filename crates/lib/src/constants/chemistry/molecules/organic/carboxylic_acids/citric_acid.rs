use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H8O7",
        name: "citric acid",
        composition: &[(6, 6), (1, 8), (8, 7)],
        molar_mass: 192.124,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(429.0),
        boiling_point_k: Some(583.0),
        density_g_cm3: Some(1.665),
    }
}
