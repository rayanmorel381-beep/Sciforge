use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6O4",
        name: "succinic acid",
        composition: &[(6, 4), (1, 6), (8, 4)],
        molar_mass: 118.088,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(458.0),
        boiling_point_k: Some(508.0),
        density_g_cm3: Some(1.560),
    }
}
