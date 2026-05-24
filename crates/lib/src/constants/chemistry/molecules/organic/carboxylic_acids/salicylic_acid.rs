use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H6O3",
        name: "salicylic acid",
        composition: &[(6, 7), (1, 6), (8, 3)],
        molar_mass: 138.122,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(431.0),
        boiling_point_k: Some(484.0),
        density_g_cm3: Some(1.443),
    }
}
