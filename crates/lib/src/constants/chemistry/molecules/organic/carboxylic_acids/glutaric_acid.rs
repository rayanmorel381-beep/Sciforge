use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H8O4",
        name: "glutaric acid",
        composition: &[(6, 5), (1, 8), (8, 4)],
        molar_mass: 132.115,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(370.7),
        boiling_point_k: Some(575.0),
        density_g_cm3: Some(1.429),
    }
}
