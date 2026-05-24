use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H4O4",
        name: "malonic acid",
        composition: &[(6, 3), (1, 4), (8, 4)],
        molar_mass: 104.061,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(409.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.619),
    }
}
