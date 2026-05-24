use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H19NO3",
        name: "morphine",
        composition: &[(1, 19), (6, 17), (7, 1), (8, 3)],
        molar_mass: 285.343,
        category: "opioid",
        state_at_stp: "solid",
        melting_point_k: Some(528.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.317),
    }
}
