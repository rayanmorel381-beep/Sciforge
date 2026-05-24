use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H15N5O11P2",
        name: "guanosine diphosphate",
        composition: &[(6, 10), (1, 15), (7, 5), (8, 11), (15, 2)],
        molar_mass: 443.201,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.300),
    }
}
