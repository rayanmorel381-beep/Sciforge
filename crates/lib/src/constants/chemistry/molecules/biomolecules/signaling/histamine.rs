use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H9N3",
        name: "histamine",
        composition: &[(6, 5), (1, 9), (7, 3)],
        molar_mass: 111.148,
        category: "neurotransmitter",
        state_at_stp: "solid",
        melting_point_k: Some(356.0),
        boiling_point_k: Some(482.0),
        density_g_cm3: Some(1.030),
    }
}
