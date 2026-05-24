use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H13N2O9P",
        name: "uridine monophosphate",
        composition: &[(6, 9), (1, 13), (7, 2), (8, 9), (15, 1)],
        molar_mass: 324.181,
        category: "nucleotide",
        state_at_stp: "solid",
        melting_point_k: Some(475.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.180),
    }
}
