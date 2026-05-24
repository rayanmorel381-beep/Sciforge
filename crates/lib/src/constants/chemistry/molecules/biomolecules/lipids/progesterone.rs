use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H30O2",
        name: "progesterone",
        composition: &[(6, 21), (1, 30), (8, 2)],
        molar_mass: 314.469,
        category: "steroid hormone",
        state_at_stp: "solid",
        melting_point_k: Some(399.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.166),
    }
}
