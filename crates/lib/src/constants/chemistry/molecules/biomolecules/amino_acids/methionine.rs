use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H11NO2S",
        name: "methionine",
        composition: &[(6, 5), (1, 11), (7, 1), (8, 2), (16, 1)],
        molar_mass: 149.211,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(554.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.34),
    }
}
