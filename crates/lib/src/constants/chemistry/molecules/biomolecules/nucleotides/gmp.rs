use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H14N5O8P",
        name: "guanosine monophosphate",
        composition: &[(6, 10), (1, 14), (7, 5), (8, 8), (15, 1)],
        molar_mass: 363.220,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.090),
    }
}
