use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H12N2O2",
        name: "ornithine",
        composition: &[(6, 5), (1, 12), (7, 2), (8, 2)],
        molar_mass: 132.161,
        category: "amino acid",
        state_at_stp: "solid",
        melting_point_k: Some(413.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.150),
    }
}
