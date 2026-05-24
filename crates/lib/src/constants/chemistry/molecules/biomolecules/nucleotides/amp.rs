use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H14N5O7P",
        name: "adenosine monophosphate",
        composition: &[(6, 10), (1, 14), (7, 5), (8, 7), (15, 1)],
        molar_mass: 347.221,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: Some(451.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.320),
    }
}
