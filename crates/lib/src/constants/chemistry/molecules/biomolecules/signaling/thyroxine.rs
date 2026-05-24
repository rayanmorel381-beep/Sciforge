use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C15H11I4NO4",
        name: "thyroxine",
        composition: &[(6, 15), (1, 11), (7, 1), (8, 4), (53, 4)],
        molar_mass: 776.870,
        category: "hormone",
        state_at_stp: "solid",
        melting_point_k: Some(504.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.314),
    }
}
