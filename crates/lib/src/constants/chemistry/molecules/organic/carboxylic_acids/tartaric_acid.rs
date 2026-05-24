use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6O6",
        name: "tartaric acid",
        composition: &[(6, 4), (1, 6), (8, 6)],
        molar_mass: 150.087,
        category: "carboxylic acid",
        state_at_stp: "solid",
        melting_point_k: Some(444.0),
        boiling_point_k: Some(548.0),
        density_g_cm3: Some(1.790),
    }
}
