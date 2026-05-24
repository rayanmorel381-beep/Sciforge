use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H8O6",
        name: "ascorbic acid",
        composition: &[(6, 6), (1, 8), (8, 6)],
        molar_mass: 176.124,
        category: "vitamin",
        state_at_stp: "solid",
        melting_point_k: Some(463.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.694),
    }
}
