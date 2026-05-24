use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H9NO2",
        name: "proline",
        composition: &[(6, 5), (1, 9), (7, 1), (8, 2)],
        molar_mass: 115.131,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(494.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.35),
    }
}
