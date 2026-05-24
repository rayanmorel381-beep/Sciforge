use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H2O4",
        name: "oxalic acid",
        composition: &[(6, 2), (1, 2), (8, 4)],
        molar_mass: 90.034,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(462.7),
        boiling_point_k: Some(638.0),
        density_g_cm3: Some(1.900),
    }
}
