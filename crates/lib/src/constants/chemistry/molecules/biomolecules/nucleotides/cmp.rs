use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H14N3O8P",
        name: "cytidine monophosphate",
        composition: &[(6, 9), (1, 14), (7, 3), (8, 8), (15, 1)],
        molar_mass: 323.197,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: Some(513.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.100),
    }
}
