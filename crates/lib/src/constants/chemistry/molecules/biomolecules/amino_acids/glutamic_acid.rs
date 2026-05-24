use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H9NO4",
        name: "glutamic acid",
        composition: &[(6, 5), (1, 9), (7, 1), (8, 4)],
        molar_mass: 147.129,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(478.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.4601),
    }
}
