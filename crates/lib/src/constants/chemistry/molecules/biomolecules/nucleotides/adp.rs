use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H15N5O10P2",
        name: "adenosine diphosphate",
        composition: &[(6, 10), (1, 15), (7, 5), (8, 10), (15, 2)],
        molar_mass: 427.201,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.490),
    }
}
