use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H14",
        name: "1-heptene",
        composition: &[(6, 7), (1, 14)],
        molar_mass: 98.186,
        category: "alkene",
        state_at_stp: "liquid",
        melting_point_k: Some(154.3),
        boiling_point_k: Some(366.78),
        density_g_cm3: Some(0.697),
    }
}
