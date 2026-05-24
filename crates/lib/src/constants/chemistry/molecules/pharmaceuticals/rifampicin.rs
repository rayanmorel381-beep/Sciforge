use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C43H58N4O12",
        name: "rifampicin",
        composition: &[(1, 58), (6, 43), (7, 4), (8, 12)],
        molar_mass: 822.953,
        category: "antibiotic",
        state_at_stp: "solid",
        melting_point_k: Some(456.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.350),
    }
}
