use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H10N2O3",
        name: "glutamine",
        composition: &[(6, 5), (1, 10), (7, 2), (8, 3)],
        molar_mass: 146.144,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(458.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.364),
    }
}
